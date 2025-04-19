/-
This is a comment with a quiz inside.

--@quiz:lean_basics
[[questions]]
type = "ShortAnswer"
prompt.prompt = "What is the keyword for definitions in Lean?"
answer.answer = "def"
context = "For example, you can write: `def x := 5`."

[[questions]]
type = "MultipleChoice"
prompt.prompt = "What symbol is used for type annotations in Lean?"
prompt.distractors = [
  "=>",
  "->",
  "=="
]
answer.answer = ":"
context = """
In Lean, we use the colon symbol to annotate types. For example: `def x : Nat := 5`
"""
--@quiz-end

And here's some more text after the quiz.
-/

def normal := 42
