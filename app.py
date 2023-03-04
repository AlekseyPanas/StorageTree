import pygame
from goals_api.goal import Manager, Goal


class App:
    def __init__(self, manager: Manager, width: int, height: int):
        self.manager = manager

        self.running = False
        self.display = None
        self.clock = pygame.time.Clock()

        self.width = width
        self.height = height

    def start(self):
        pygame.init()
        self.display = pygame.display.set_mode((self.width, self.height), pygame.DOUBLEBUF)
        self.running = True
        self.__run()

    def __run(self):
        while self.running:
            # Clear display
            self.display.fill((0, 0, 0))

            # Execute events
            for e in pygame.event.get():
                if e.type == pygame.QUIT:
                    self.stop()

            # Update display
            pygame.display.update()
            self.clock.tick(90)

            # Update caption
            pygame.display.set_caption(str(self.clock.get_fps()))

    def __render_timetable(self):
        pass
        # TODO: Render the timetable visual on self.screen

    def stop(self):
        self.running = False
