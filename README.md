# Mozilla Study Group Lessons

Let's build a collection of introductory lessons for short tutorials on scientific coding and data management people can deliver to a study group in their labs and departments.

### What's a Mozilla Study Group and How do I Start One?

Mozilla Study Groups are a great casual way to get people from your field together to share their skills in computing and data management, in a relaxed, no-pressure environment with friends and colleagues, and to create a space where it's okay to ask any question you like. The goal should be to help everyone get some hands-on practice with a new idea or tool they can use in their every-day research, and support each other as we learn.

Want to start your own? Check out our [how to guide](http://mozillascience.github.io/studyGroupHandbook/), and our quick-start [Mozilla Study Group website kit](https://github.com/mozillascience/studyGroup).
 
### Where We Are

People are starting coding meetups for scientists and researchers around the world, both with us and on their own. Check out the [map of Study Groups](https://github.com/mozillascience/studyGroupLessons/blob/master/whereWeAre.geojson), see if there's one near you, or add your own by sending us a pull request (add yourself to `whereWeAre.geojson`, link required!), or open an issue and we'll add you.

For help getting your longitude and lattitude pre-formatted in geojson, find yourself on [this awesome web app](http://dbsgeo.com/latlon/).

<script src="https://embed.github.com/view/geojson/mozillascience/studyGroupLessons/master/whereWeAre.geojson"></script>

### What is a Study Group Lesson Like?

A good Study Group lesson should be a tutorial that's *as hands on as possible* - an hour-long lecture just doesn't have the same effect. To create a successful tutorial, keep the following in mind:

 - Give participants challenge questions to try out their new skills as often as possible; never lecture for more than five minutes at a stretch.
 - Go as slow as the participants need; never rush to get through all the material.
 - Encourage people to work together and chat about what they're learning and how they might use it.
 - Try to come up with examples that are relevant and interesting from your field of study.

Have a look in the template directory of this repository for some more details.

### Contribution Guidelines

If you'd like to contribute a new lesson, write it up in a new repo following these guidelines:
 - put the lesson script in `README.md`, with your name, subject area and lesson topic at the top.
 - make sure to include an open source `LICENSE` file.
 - make sure all data / examples / figures / anything else you want are all included in your repo.
 - Here's an [example](https://github.com/BillMills/pythonPackageLesson) of all that.

Then, add your repo to this one as a Submodule.

#### Using Submodules

Submodules are super handy for making a loose federation of resources like these, but are relatively rarely used; here's a primer.

##### Adding a new lesson

In this repo, do

```
git submodule add <url of your new lesson repo>
```

then commit, push and pull request as usual. That's it!

##### Pulling down lessons locally

Clone this repo as usual:

```
git clone https://github.com/mozillascience/studyGroupLessons
```

You'll see directories for all the different lessons, but they'll be empty. Tell git you're going to be using submodules (only need to do this once for this project):

```
git submodule init
```

Then, you can pull down individual submodules like so (replace `awk-lesson` with the name of the submodule you want):

```
git submodule update awk-lesson
```

Finally, pull down new changes to lessons in future via:

```
git pull upstream master
git submodule update awk-lesson
```

##### Updating Submodule Pointers

(Note: maintainers of this repo will do this automatically regularly - contributors shouldn't need to do this)

```
git submodule update --remote
```

#### Need Help?

**Not comfortable with Markdown or Git?** That's ok! If Markdown isn't your thing, just cut and paste your notes into a plain text file, and we'll help you format them; if you're unsure of git, open a new [issue](https://github.com/mozillascience/studyGroupLessons/issues) and we'll help get you included.

**Don't have a local Mozilla Study Group?** That's also ok! We're collecting and sharing lessons from everyone who would like to write one, whether they are involved with a Mozilla Study Group or not. And if you'd like to start your own Mozilla Study Group, don't miss the [how-to guide](http://mozillascience.github.io/studyGroupHandbook/).

### Check Out Our Friends!

There are groups that have been meeting & sharing their work for some time now - check out the [list in the Mozilla Study Group Handbook](http://mozillascience.github.io/studyGroupHandbook/lessons.html#lessons), and send us your own if you'd like us to help share your work.
