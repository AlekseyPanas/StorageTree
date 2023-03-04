from __future__ import annotations
from cli.commands import CommandFactory, Command, CommandNotFound, CommandExecutionError


class App:
    def __init__(self):
        self.running = False

    def start(self):
        self.running = True
        self.__run()

    def __run(self):
        while self.running:
            inp = input("storage_tree $ ")

            try:
                command = CommandFactory.get_command(self, inp)
                command.execute()

            except CommandNotFound:  # Cmd not found exception
                print("command not found")

            except CommandExecutionError as e:  # Parent exception for a command processing exception
                print(e.get_message())

    def stop(self):
        self.running = False
