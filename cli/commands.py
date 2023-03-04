from __future__ import annotations
import cli.cli
from abc import abstractmethod
import re
from typing import Optional


class CommandNotFound(Exception):
    pass


class CommandExecutionError(Exception):
    def __init__(self, message: str):
        self.__message = message
        super().__init__(message)

    def get_message(self):
        return self.__message


class Command:
    def __init__(self, app: cli.cli.App, *args):
        self.app = app
        self.args = args

    @abstractmethod
    def execute(self):
        """Execute this command"""


class QuitCommand(Command):
    def execute(self):
        self.app.stop()


class CommandFactory:
    __COMMANDS = {
        "quit": QuitCommand
    }

    @staticmethod
    def get_command(app: cli.cli.App, user_input: str) -> Optional[Command]:
        """
        Converts user input to a command
        :param app: The cli app which called this factory
        :param user_input: String that a user wrote in the console, raw and unprocessed
        :return: a Command object if valid, an error if invalid, or None if input was empty
        """
        user_input = re.sub(r'\s+', ' ', user_input.strip()).split(" ")
        if len(user_input) > 0:
            cmd_str = user_input[0].lower()

            try:
                cmd = CommandFactory.__COMMANDS[cmd_str](app, *user_input[1:])
            except KeyError as e:
                raise CommandNotFound

            return cmd

        return None
