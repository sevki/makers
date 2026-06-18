all: slow fast

slow:
	@sleep 1
	@echo slow-done

fast:
	@false
