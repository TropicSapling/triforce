import sublime
import sublime_plugin

import re

funcs      = set()
funcs_iter = iter(funcs)

ready = True

prelude = {"if", "then", "else", "unless", "for each", "while", "break", "continue",
           "continue matching", "continue matching for", "return", "eval", "prerun",
           "run", "async scope", "spawn", "as", "fulfilling", "where", "where we",
           "which", "match", "matches", "is", "is a", "is an", "is any", "are",
           "could be", "export", "import", "export all", "import all", "except",
           "from", "into", "expose", "with", "all", "in", "excl", "any",
           "any suitable", "any of", "optionally", "recollected", "listified",
           "codified", "stringified", "ensure", "print", "println", "macro"}

def between(view, a, b):
	return view.substr(sublime.Region(a, b))

def undo(view):
	print("CUSTOM UNDO")
	view.run_command('undo')

class TriUndoCommand(sublime_plugin.TextCommand):
	def run(self, edit):
		# Note: below timeout needed because Sublime Text is weird
		sublime.set_timeout(lambda: undo(self.view), 0)

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
						if s != "" and s not in prelude: funcs.add(s)
				
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
		view.insert(edit, 0, 'Q')
		view.erase(edit, sublime.Region(0, 1))
	
	def run(self, edit):
		global ready
		
		self.highlight_calls(edit, self.view.size())
		ready = True

class FuncListener(sublime_plugin.EventListener):
	def on_modified_async(self, view):
		global ready
		
		# Need 'try' in case attempting to run after file closed
		try:
			if ready and view.syntax().scope == 'source.triforce':
				ready = False
				# TODO: fix running command messing with Ctrl+Z
				# view.run_command('highlight_tri_func_call')
		except AttributeError: ()
