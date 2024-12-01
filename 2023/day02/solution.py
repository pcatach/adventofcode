class GameSample:
    def __init__(self, red: int = 0, green: int = 0, blue: int = 0):
        self.red = red
        self.green = green
        self.blue = blue

    @classmethod
    def from_string(cls, sample_string: str) -> "GameSample":
        sample_parameters = {}
        for and_color in sample_string.split(", "):
            num, color = and_color.split(" ")
            sample_parameters[color] = int(num)
        return cls(**sample_parameters)

    def __repr__(self):
        return f"<R: {self.red} G: {self.green} B: {self.blue} >"


class Game:
    def __init__(self, id, game_samples: list[GameSample]):
        self.id = id
        self.game_samples = game_samples

    def max(self) -> tuple[int, int, int]:
        max_red = max(sample.red for sample in self.game_samples)
        max_green = max(sample.green for sample in self.game_samples)
        max_blue = max(sample.blue for sample in self.game_samples)
        return (max_red, max_green, max_blue)

    def __repr__(self):
        return f"<id: {self.id} samples: {self.game_samples}>"


class GameConfiguration:
    def __init__(self, red, green, blue):
        self.red = red
        self.green = green
        self.blue = blue

    def is_game_sample_possible(self, game_sample: GameSample) -> bool:
        return all(
            (
                game_sample.red <= self.red,
                game_sample.green <= self.green,
                game_sample.blue <= self.blue,
            )
        )

    def is_game_possible(self, game: Game) -> bool:
        return all(
            self.is_game_sample_possible(game_sample)
            for game_sample in game.game_samples
        )

    def power(self) -> int:
        return self.red * self.green * self.blue

    def __repr__(self):
        return f"<(conf) R: {self.red} G: {self.green} B: {self.blue}>"


def read_games(input_file_path) -> list[Game]:
    with open(input_file_path) as input_file:
        game_strings = input_file.read().split("\n")
    games = []
    for game_string in filter(None, game_strings):
        game_id, sample_strings = game_string.split(": ")
        id = int(game_id.replace("Game ", ""))
        game_samples = []
        for sample_string in sample_strings.split("; "):
            game_samples.append(GameSample.from_string(sample_string))
        games.append(Game(id, game_samples))
    return games


if __name__ == "__main__":
    games = read_games("input.txt")
    conf = GameConfiguration(12, 13, 14)

    print()
    print(sum(game.id for game in games if conf.is_game_possible(game)))
    print()

    powers = 0
    for game in games:
        minimal_conf = GameConfiguration(*game.max())
        powers += minimal_conf.power()
    print(powers)
