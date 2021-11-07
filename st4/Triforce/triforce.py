import sublime
import sublime_plugin

import re
import time

sublime.Region.__hash__ = lambda self: hash(str(self))

alltime_highlights = highlights = set()

funcs      = set()
funcs_iter = iter(funcs)

ready = True

# Remember to update this as syntax changes
keyword_ctrl       = r'\b(if|then|else|unless|(for\s+)?each|while|break|continue(\s+matching(\s+for)?)?|return|eval|prerun|run|spawn|assign|to\s+worker\s+in|defer)\b'
keyword_check      = r'\b(as|fulfilling|where(\s+we)?|which|when|matches|and|is(\s+(a|an|any)(?!{{identifier}}))?|are|could\s+be)\b'
keyword_namespace  = r'\b((ex|im)port(\s+all)?|except|from|into|expose)\b'
keyword_type       = r'\b(bool|nat|int|frac|complex|num|str)\b'
keyword_type_spec  = r'\b(impure|unpredictable|macro|implic\s+made|suitable\s+of|ref\s+to|allowing|parsed|raw|cloaked|constructed\s+using|unsafe|async\s+escaping|exclusively)\b'
keyword_misc       = r'\b(with|all|in|excl|any(\s+suitable|of)?|optionally|recollected|listified|codified|stringified|ensure|print(ln|err)?|otherwise)\b'

prelude = (
	keyword_ctrl      + r'|' +
	keyword_check     + r'|' +
	keyword_namespace + r'|' +
	keyword_type      + r'|' +
	keyword_type_spec + r'|' +
	keyword_misc
)

def between(view, a, b):
	return view.substr(sublime.Region(a, b))

# This exists because Python
def setReady(val):
	global ready

	ready = val

class TriHighlighter(sublime_plugin.EventListener):
	def find_funcs(self, view):
		global funcs, funcs_iter

		size = view.size()

		for sel in view.sel():
			i   = min(sel.a, sel.b) - 256
			end = max(sel.a, sel.b) + 256

			if i   < 0    : i   = 0
			if end > size : end = size

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

	def highlight_calls(self, view, start_time):
		global highlights, alltime_highlights

		try:
			f = next(funcs_iter)
		except StopIteration:
			self.dehighlight_calls(view)
			self.find_funcs(view)
			return

		# TODO: highlight longest function name first
		regex     = r"\b" + re.escape(f) + r"\b"
		fcall_pos = [m.span() for m in re.finditer(regex, between(view, 0, view.size()))]

		for (a, b) in fcall_pos:
			scope = view.scope_name(a)
			if ("comment"                       in scope or
				"string"                        in scope or
				"variable"                      in scope or
				"operator"                      in scope or
				"entity.name.function.triforce" in scope):
				continue

			highlight = sublime.Region(a, b)
			highlights.add(str(highlight))
			alltime_highlights.add(str(highlight))
			view.add_regions(
				key     = str(highlight),
				regions = [highlight],
				scope   = "variable.function.triforce", # TODO: support CC & CS
				flags   = sublime.DRAW_NO_OUTLINE|sublime.PERSISTENT
			)

		# Keep going for up to 50 ms.
		if time.time() - start_time < 0.05:
			self.highlight_calls(view, start_time)

		# Trigger on_modified_async(...) to restart
		# TODO: fix this causing file to always say it's unsaved
		#       - or actually probably better to not run in the background
		# view.insert(edit, 0, 'Q')
		# view.erase(edit, sublime.Region(0, 1))

	def dehighlight_calls(self, view):
		global highlights

		for highlight in alltime_highlights:
			# Dehighlight if no longer valid
			if highlight not in highlights:
				view.erase_regions(highlight)

		highlights = set()

	def on_modified_async(self, view):
		# Need 'try' in case attempting to run after file closed
		try:
			if ready and view.syntax().scope == 'source.triforce':
				setReady(False)
				self.highlight_calls(view, time.time())
				setReady(True)
		except AttributeError: ()
