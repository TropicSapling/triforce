import sublime
import sublime_plugin

import re

funcs = set()
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

def pythonLambdasDontSupportAssignmentApparently():
	global ready
	
	ready = True

class HighlightTriFuncCallCommand(sublime_plugin.TextCommand):
	def find_funcs(self):
		global funcs
		
		view = self.view
		size = view.size()
		
		i = 0
		while i < size:
			while "comment" in view.scope_name(i): i += 1
			
			if "storage.type" in view.scope_name(i) and between(view, i, i+5) == "func ":
				s = ""
				while view.substr(i) != '\n' and i < size:
					while "entity.name.function" not in view.scope_name(i) and i < size:
						if view.substr(i) == '\n': break
						i += 1
					
					start = i
					
					while "entity.name.function" in view.scope_name(i):
						if view.substr(i) == '\n': break
						i += 1
					
					if i < size: s += between(view, start, i) + " "
				
				s = s.rstrip()
				if view.substr(i - 1) == ';' or view.substr(i - 1) == '{':
					if s != "" and s not in prelude: funcs.add(s)
			
			i += 1
		
		# print(funcs) # DEBUG
	
	def highlight_calls(self, edit):
		view = self.view
		size = view.size()
		
		for f in funcs:
			regex     = r"\b" + re.escape(f) + r"\b"
			fcall_pos = [m.span() for m in re.finditer(regex, between(view, 0, size))]
			
			offset = 0
			for (a, b) in fcall_pos:
				scope = view.scope_name(a + offset)
				if ("comment"                       in scope or
					"string"                        in scope or
					"variable"                      in scope or
					"entity.name.function.triforce" in scope):
					continue
				
				if view.substr(a + offset) != '\u200b':
					view.insert(edit, a + offset, '\u200b')
					view.insert(edit, b + offset + 1, '\u200b')
					size   += 2
					offset += 2
	
	def run(self, edit):
		self.find_funcs()
		self.highlight_calls(edit)

class FuncListener(sublime_plugin.EventListener):
	def on_modified_async(self, view):
		global ready
		
		if ready and view.window().extract_variables()["file_extension"] == "tri":
			ready = False
			view.run_command('highlight_tri_func_call')
			# Don't update more often than every 5 seconds
			sublime.set_timeout_async(pythonLambdasDontSupportAssignmentApparently, 5000)
			# TODO: use timeout to split up things to make it less laggy
