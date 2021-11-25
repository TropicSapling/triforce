import re

from functools import partial
from time      import time

from sublime        import Region, DRAW_NO_OUTLINE, PERSISTENT
from sublime_plugin import EventListener

Region.__hash__ = lambda self: hash(str(self))

alltime_highlights = highlights = set()

funcs      = set()
funcs_iter = iter(funcs)

ready = True

# Remember to update this as syntax changes
keyword_ctrl       = r'\b(if|then|else|unless|(for\s+)?each|while|break|continue(\s+matching(\s+for)?)?|return|eval|prerun|run|spawn|assign|to\s+worker\s+in|defer)\b'
keyword_check      = r'\b(as|fulfilling|where(\s+we)?|which|when|matches|and|is(\s+(a|an|any)(?!{{identifier}}))?|are|could\s+be)\b'
keyword_namespace  = r'\b((ex|im)port(\s+all)?|except|from|into|expose)\b'
keyword_type       = r'\b(type|bool|nat|int|frac|complex|num|str)\b'
keyword_type_spec  = r'\b(impure|unpredictable|macro|implic\s+made|(suitable|subtype)\s+of|ref\s+to|allowing|parsed|raw|cloaked|constructed\s+using|unsafe|async\s+escaping|exclusively)\b'
keyword_misc       = r'\b(panic|with|all|in|excl|any(\s+(suitable|of))?|optionally|recollected|listified|codified|stringified|ensure|print(ln|err)?|otherwise|mod)\b'

prelude = (
	keyword_ctrl      + r'|' +
	keyword_check     + r'|' +
	keyword_namespace + r'|' +
	keyword_type      + r'|' +
	keyword_type_spec + r'|' +
	keyword_misc
)

def between(view, a, b):
	return view.substr(Region(a, b))

# This exists because Python
def setReady(val):
	global ready

	ready = val

def locally(view, f):
	size = view.size()

	for sel in view.sel():
		i   = min(sel.a, sel.b) - 256
		end = max(sel.a, sel.b) + 256

		if i   < 0    : i   = 0
		if end > size : end = size

		f(view, i, end)

class TriHighlighter(EventListener):
	def find_funcs(self, view, i, end):
		global funcs, funcs_iter

		while i < end:
			while "comment" in view.scope_name(i): i += 1

			if "storage.type" in view.scope_name(i) and between(view, i, i+5) == "func ":
				# Found function definition; register

				s = ""
				while view.substr(i) != '\n' and i < end:
					while "entity.name.function" not in view.scope_name(i) and i < end:
						if view.substr(i) == '\n': break
						i += 1

					start = i

					while "entity.name.function" in view.scope_name(i):
						if view.substr(i) == '\n': break
						i += 1

					if i < end: s += between(view, start, i) + " "

				s = s.rstrip()
				if view.substr(i - 1) == ';' or view.substr(i - 1) == '{':
					if s != "" and s[0].islower() and re.match(prelude, s) == None:
						funcs.add(s)

			i += 1

		funcs_iter = iter(funcs)

		# print(funcs) # DEBUG

	def find_local_funcs(self, view):
		locally(view, self.find_funcs)

	def find_all_funcs(self, view):
		self.find_funcs(view, 0, view.size())

	def highlight_calls(self, start_time, view, start, end):
		global highlights, alltime_highlights

		try:
			f = next(funcs_iter)
		except StopIteration:
			self.dehighlight_calls(view, start, end)
			self.find_local_funcs(view)
			return

		regex     = r"\b" + re.escape(f) + r"\b"
		fcall_pos = [(m.start() + start, m.end() + start) for m in re.finditer(regex, between(view, start, end))]

		for (a, b) in fcall_pos:
			scopes = view.scope_name(a)
			scope  = "variable.function.triforce"

			if "comment" in scopes or "string" in scopes:
				if "meta.cc" in scopes and "string" not in scopes:
					scope = "cc." + scope
				elif "meta.cs" in scopes and "comment" not in scopes:
					scope = "cs." + scope
				else:
					continue

			if ("variable"             in scopes or
				"operator"             in scopes or
				"entity.name.function" in scopes):
				continue

			highlight = Region(a, b)
			highlights.add(str(highlight))
			alltime_highlights.add(str(highlight))
			view.add_regions(
				key     = str(highlight),
				regions = [highlight],
				scope   = scope,
				flags   = DRAW_NO_OUTLINE|PERSISTENT
			)

		# Keep going for up to 50 ms.
		if time() - start_time < 0.05:
			self.highlight_calls(start_time, view, start, end)

		# Trigger on_modified_async(...) to restart
		# TODO: fix this causing file to always say it's unsaved
		#       - or actually probably better to not run in the background
		# view.insert(edit, 0, 'Q')
		# view.erase(edit, Region(0, 1))

	def highlight_local_calls(self, view):
		locally(view, partial(self.highlight_calls, time()))

	def highlight_all_calls(self, view):
		self.highlight_calls(time(), view, 0, view.size())

	def dehighlight_calls(self, view, start, end):
		global highlights

		for highlight in alltime_highlights:
			# Dehighlight if no longer valid
			if highlight not in highlights:
				regions = view.get_regions(highlight)
				if len(regions) > 0 and regions[0].a >= start and regions[0].b <= end:
					view.erase_regions(highlight)

		highlights = set()

	def on_modified_async(self, view):
		# Need 'try' in case attempting to run after file closed
		try:
			if ready and view.syntax().scope == 'source.triforce':
				setReady(False)
				self.highlight_local_calls(view)
				setReady(True)
		except AttributeError: ()
