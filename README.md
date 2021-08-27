# Chesshound

Chesshound is a rust library and cli tool that is intended for analyzing player patterns within any set of games. Its main goals include:

- "At a glance" analysis of player mistakes through the use of opening statistics and blunder patterns.
- Easy scraping of player games from chess.com or lichess.org based on useful criteria such as time period.
- Preprocessing of game data such as annotating with engine analysis.

The current functionality being implemented is a more powerful opening explorer than provided by chess.com or lichess, which allows for games to be sampled in much more ways than currently provided.

Farther down the road is a goal to make the application more accessible by:

- Creating a python wrapper around library functionality.
- Using the python library to implement a GraphQL service for the application.
- Designing a ReactJS frontend for end users who wish to analize their games or use the service to extract game data.
