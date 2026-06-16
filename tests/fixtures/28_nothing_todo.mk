# Exercises the commands_started counter (now an atomic): 'noop' has neither a
# recipe nor prerequisites, so make's update finds commands_started unchanged
# and prints "Nothing to be done for 'noop'."; 'done' has a recipe, so the
# counter advances and the recipe runs. Byte-checked against the C oracle.
done:
	@echo did-something

noop:
