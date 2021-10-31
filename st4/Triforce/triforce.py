import sublime
import sublime_plugin

import re

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

def do(view, cmd):
	setReady(False)

	# Undo/Redo syntax highlighting
	last_cmd = view.command_history(0)[0]
	# TODO: fix so 'i < 256' not needed (check properly if at end)
	i = 0
	while ('tri_' in last_cmd or last_cmd == '') and i < 256:
		view.run_command(cmd)
		last_cmd = view.command_history(0)[0]
		i += 1

	# Undo/Redo what the user typed
	view.run_command(cmd)

	# Undo/Redo final syntax highlighting
	last_cmd = view.command_history(1)[0]
	if view.command_history(1)[0] == 'highlight_tri_func_call':
		view.run_command(cmd)

	sublime.set_timeout_async(lambda: setReady(True), 0)

class TriUndoCommand(sublime_plugin.TextCommand):
	def run(self, edit):
		# Note: below timeout needed because Sublime Text is weird
		sublime.set_timeout_async(lambda: do(self.view, 'undo'), 0)

class TriRedoCommand(sublime_plugin.TextCommand):
	def run(self, edit):
		# Note: below timeout needed because Sublime Text is weird
		sublime.set_timeout_async(lambda: do(self.view, 'redo'), 0)

class HighlightTriFuncCallCommand(sublime_plugin.TextCommand):
	def find_funcs(self, edit):
		global funcs
		global funcs_iter

		view = self.view
		size = view.size()

		for sel in view.sel():
			i   = min(sel.a, sel.b) - 256
			end = max(sel.a, sel.b) + 256

			if i   < 0    : i   = 0
			if end > size : end = size

			while i < end:
				while "comment" in view.scope_name(i): i += 1

				if view.substr(i) == '\u200b':
					# Found existing highlight; dehighlight if no longer valid

					i += 1
					start = i
					while i < end and view.substr(i) != '\u200b':
						i += 1

					found_match = False
					for f in funcs:
						regex = r"\b" + re.escape(f) + r"\b"
						term  = between(view, start-2, i+2).replace('\u200b', '')

						if re.search(regex, term):
							found_match = True
							break

					if not found_match:
						view.erase(edit, sublime.Region(start-1, start))
						view.erase(edit, sublime.Region(i-1, i))
						i   -= 2
						end -= 2

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

	def highlight_calls(self, edit, size):
		view = self.view

		try:
			f = next(funcs_iter)
		except StopIteration:
			self.find_funcs(edit)
			return

		# TODO: highlight longest function name first
		regex     = r"\b" + re.escape(f) + r"\b"
		fcall_pos = [m.span() for m in re.finditer(regex, between(view, 0, size))]

		offset = 0
		for (a, b) in fcall_pos:
			scope = view.scope_name(a + offset)
			if ("comment"                       in scope or
				"string"                        in scope or
				"variable"                      in scope or
				"operator"                      in scope or
				"entity.name.function.triforce" in scope):
				continue

			if view.substr(a + offset - 1) != '\u200b':
				view.insert(edit, a + offset, '\u200b')
				view.insert(edit, b + offset + 1, '\u200b')
				# TODO: fix inserted characters messing with backspace & arrow keys
				size   += 2
				offset += 2

		# Trigger on_modified_async(...) to restart
		# TODO: fix this causing file to always say it's unsaved
		#       - or actually probably better to not run in the background
		# view.insert(edit, 0, 'Q')
		# view.erase(edit, sublime.Region(0, 1))

	def run(self, edit):
		setReady(False)
		self.highlight_calls(edit, self.view.size())
		setReady(True)

class FuncListener(sublime_plugin.EventListener):
	def on_modified_async(self, view):
		# Need 'try' in case attempting to run after file closed
		try:
			if ready and view.syntax().scope == 'source.triforce':
				view.run_command('highlight_tri_func_call')
		except AttributeError: ()
