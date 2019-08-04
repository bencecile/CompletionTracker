# Completion Tracker
This is the English version.

日本語版は[こちにクリック](./README-JP.md)してください。

---

This goal of this application is to let you track your completion of pretty much anything. It could be TV Shows, books, movies, anime or games. This project is an application that employs a web interface, so you should spin up your own and use it however you like. It would be nice however, if you made an issue for a maintainer to create a source so that a font of information can be found here, for everyone.

Monopolizing this information is a complete non-goal, as it should be open and usable (like Wikipedia).

Although this helps with a single person's completion tracking, it also handles series' information compilation. If there is a cinematic universe or book series that is gigantic, I want this application to be a reliable font of information to see how everything's connected.

Multiple people can use the completion tracking at the same time. It would be recommended to do this only with other people you know well, as there is no login system.

A cool idea would be to set up this server somewhere, with no tracking enabled, as a regular web server just so that the information on the series' can be easily accessed.

## Creating an issue
If something isn't working, you spot a bug, or you think of a cool addition, please create an Issue so that anything can be fixed/reviewed. If a problem is unknown, it's impossible to make a solution for it.

Even if it's something small like a typo, or some wording is confusing, I would love to know about it.

If there's something that you've been tracking but it isn't a proper Source yet, please feel free to create an Issue or email me (you can find contact information on [my profile](https://github.com/bencecile)).

## Jargon
There are a bunch of terms that are used throughout, so their meaning is clearly identified here.

**Arc** - A grouping of sources within a series because they have related story elements that are different than other sources not in the same Arc. They are also directly related. (eg. Ezio's 3 games in Assassin's Creed is an arc,and each one leads into the next, but the first 4 games are focused on Desmond's series)

**Series** - A collection of sources that are related, somehow. It could be the setting, gameplay, or characters. The titles are usually similar. (eg. Assassin's Creed has many games, but not all of them are directly related)

**Source** - The original source material (original work) like a movie, game, or book. (eg. A single Assassin's Creed game)

**Tracker** - A profile to track completion data for a single user. See [Tracking](#Tracking) for more information.

**Universe** - A collection of series that are related because they all happen in the same (artificial) world. The series in the universe will be disjointed but play on the same basic principles (eg. Brandon Sanderson's Cosmere, or Marvel)

## Tracking
Don't worry, this isn't your malicious "hack your data to steal your credentials" kind of tracking.

What we're dealing with is to soley track completion. This could be the completion of nearly anything, from books to movies, TV shows to games, this application has been created with flexibility in mind.

### Game Tracking
Games are a little bit special. TODO

## Project Layout
All information pertaining to the sources will be in the JSON file format.

This will ideally be created within the app, and then written out to these JSON files.

### Run Info
This is info that needs to be specified for the server to run. Anything optional will have the `Option` type. The full specification can be found in the [run_info.rs](./src/run_info.rs) file in the RunInfo and TrackerInfo structs.

When specifying the tracking info, only a single tracker may leave the `tracking_file` field null.

A sample is given here: TODO

### Sources
This is the folder that holds the information on the source material. This could be books, games, movies.

It holds data about each source, like it's original release date for different regions, author, summary, etc.

This application should ideally be able to load everything on startup, keeping everything in memory for each request.

### Series
This holds the information on each series, such as related sources and ordering of sources. This is kept separate from the actual sources so that it doesn't become a web of nightmares to keep track of everything.

## Site Data
To support the open-ness of the application, any user should be able to download their data into a format that would be easy to use or import elsewhere (JSON or CSV).

We should be able to re-import user information

It should be optional to download the information related to the sources.

## Running the Program
### Running a pre-compiled version
I plan on making a release of the Completion Tracker with every major and minor release. Right now, I will only be doing this for Windows. If there is enough demand for other platforms (eg. Linux or Mac), I should be able to figure something out.

TODO

### Building from source code
To build and run the program, you will need:
1. To have Rust installed
TODO

## Connecting from other devices
TODO
