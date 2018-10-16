# Complete Tracker
This goal of this application is to let you track your completion of pretty much anything.
It could be TV Shows, books, movies, even anime or games.

This project is a web server, so you can spin up your own and use it however you like.
Monopolizing this information is a complete non-goal, and it should be open and usable
(like Wikipedia).

Although this helps with user completion tracking, it also does series information compilation.
If there is a cinemeatic universe or book series that is gigantic, I want this application
to be a reliable font of information to see how everything's connected.

## Jargon
There are a bunch of terms that are used throughout the application, so their meaning should
be clearly identified.

**Arc** - A grouping of sources within a series because they have related story elements that
are different than other sources not in the same Arc.

**Series** - A collection of sources that are related, somehow. The titles are usually similar.

**Source** - The original source material (original work) like a movie or game.

**Universe** - A collection of series that are related because they all happen in the same
(artificial) world.

## Project Layout
Any information that needs to be stored will be in the JSON format. This is so that
non-programmers can contribute, and so it's easier to make changes.

There shouldn't be a performance hit beyond start-up time, as it will either keep all of the
information in memory, or store it in a database that can be quickly fetched.

### Sources
This is the folder that holds the information on the source material.
This could be books, games, movies.

It holds data about each source, like it's original release date in different regions,
author, summary, etc.

Everything should be sorted and organized logically, so that the program can consume it on startup.
(ie. keep series of the same medium together).
Series information should be separate from

### Series
This holds the information on each series, such as related sources and ordering of sources.
This is kept separate from the actual sources so that it doesn't become a web of nightmares
to keep track of everything.
