from goals_api.goal import Goal, Manager
import app
from cli import cli

app.App(Manager(), 1000, 800).start()

# cli.App().start()
