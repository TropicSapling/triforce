import sublime
import sublime_plugin

import re

from threading import Timer

funcs = set()
ready = True

prelude = {"if", "then", "else", "unless", "for each", "while", "break", "continue",
           "continue matching", "continue matching for", "return", "eval", "prerun",
           "run", "async scope", "spawn", "as", "fulfilling", "where", "where we",
           "which", "match", "matches", "is", "is a", "is an", "is any", "are",
           "could be", "export", "import", "export all", "import all", "except",
           "from", "into", "expose", "with", "all", "in", "excl", "any",
           "any suitable", "any of", "optionally", "recollected", "listified",
           "codified", "stringified", "ensure", "print", "println"}

def pythonLambdasDontSupportAssignmentApparently():
	global ready
	
	ready = True

class HighlightTriFuncCallCommand(sublime_plugin.TextCommand):
	def run(self, edit):
		view = self.view
		
		lines = view.lines(sublime.Region(0, self.view.size()))
		for view_line in lines:
			line   = view.substr(view_line)
			offset = view_line.to_tuple()[0]
			
			s = ""
			
			i = 0
			while i+5 <= len(line):
				if line[i:i+2] == "//": break
				
				if line[i:i+5] == "func ":
					a = offset + i
					b = a
					while b - offset < len(line):
						a = b
						while "entity.name.function.triforce" not in view.scope_name(a):
							a += 1
							if a - offset >= len(line): break
						
						if a - offset >= len(line): break
						
						b = a
						while "entity.name.function.triforce" in view.scope_name(b):
							b += 1
							if b - offset >= len(line): break
						
						s += view.substr(sublime.Region(a, b)) + " "
						
						i = b - offset
						while i < len(line) - 1:
							i += 1
					
					if line[i] != ';' and line[i] != '{':
						break
					elif len(s) > 0 and s[:-1] not in prelude:
						funcs.add(s[:-1])
				
				i += 1
		
		lines = view.lines(sublime.Region(0, self.view.size()))
		for view_line in lines:
			line   = view.substr(view_line)
			offset = view_line.to_tuple()[0]
			
			for f in funcs:
				regex = r"\b" + re.escape(f) + r"\b"
				fcall_pos = [m.span() for m in re.finditer(regex, line)]
				
				i = 0
				for (a, b) in fcall_pos:
					scope = view.scope_name(offset + a)
					
					ok = ("comment"                       not in scope and
					      "string"                        not in scope and
					      "entity.name.function.triforce" not in scope and
					      "variable"                      not in scope)
					
					if ok and view.substr(offset + a - 1) != '\u200b':
						# Ensure file hasn't changed while doing previous calculations
						fcall_pos_now = [m.span() for m in re.finditer(regex, view.substr(view_line))]
						if len(fcall_pos_now) >= i + 1 and (a, b) == fcall_pos_now[i]:
							view.insert(edit, offset + a, '\u200b')
							view.insert(edit, offset + b + 1, '\u200b')
					
					i += 1
		
		print(funcs)

class FuncListener(sublime_plugin.EventListener):
	def on_selection_modified_async(self, view):
		global ready
		
		if ready and view.window().extract_variables()["file_extension"] == "tri":
			ready = False
			view.run_command('highlight_tri_func_call')
			# Don't update more often than every 5 seconds
			t = Timer(5, pythonLambdasDontSupportAssignmentApparently)
			t.start()
			# TODO: use timer to split up things to make it less laggy
