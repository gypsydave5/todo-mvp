# Todo-MVP

The objective of this project is to demonstrate that it is _relatively_ simple
to build web applications using HTML, CSS and a small amount of server side
code written in a language of your choice.

It's the Todo [_Minimum Viable Product_][MVP] - the simplest and most
extensible application you can write - but perhaps it's also the [_Most
Valuable Player_][MVPlayer] in your web development toolkit. I like to think so!

## META-TODO

- [x] Working Todo-MVP application
- [x] Nice CSS
- [x] Good a11y
- [x] Simple acceptance test
- [ ] Best in class a11y
- [ ] Implement in multiple languages
- [ ] Multiple CSS files
- [ ] Automated deployment
- [ ] Automate the acceptance test
- [ ] ???
- [ ] PROFIT!

## The Todo Application

The project consists of the following:

  - Many Todo application, written in multiple languages, all
  each serving the same HTML and implementing the same API.
  - An acceptance test to confirm that the application does the above
  - This README

## Principles

Whereas I respect the skill and effort that has been put into developing client
side browser applications in JavaScript, it's my contention that a good deal of
that functionality can be achieved by the correct application of the tried and
tested technologies of HTML and CSS, especially in the modern versions, [CSS3][CSS3]
and [HTML 5][HTML5].

I also like many of the server side frameworks that are used to write web
applications in - Ruby on Rails, Spring, Django for example. But I also believe
that for many of the simple (and not so simple) applications that we build on
the Web we do not need them.

That is to say that, while I think these frameworks have their place, they are
for the most part cumbersome and obfuscating, often no better than using simpler
libraries and technologies and often a lot worse.

The subject of this demonstration is the Todo app, implemented in a server side
language and presented in HTML and CSS.

## What this is not

This is not a Luddite call to smash the frames and go back to hand weaving the
Web. That would be stupid. Rather, the idea is to _keep it simple_. The simplest
possible solution to writing a todo application isn't a [single page app][SPA];
it's written in server side code.

'Simple' does not mean we go back to banging the bytes together in raw assembly
code, or that we 'build our own framework' (whatever that means).

## Constraints

The following constraints are followed when writing an implementation:

### No JavaScript

I love JavaScript - I've written it in all of my work and a lot in my spare
time. It's wonderful, and fun, and empowering - warts and all. But it's not
necessary to write a simple Todo application, and so we won't use it here.

### No Frameworks

By 'framework' I mean software which requires certain folder structures,
configuration files, special commands to run it, and a vague feeling that you're
working in magickal environment in which the software you're writing is not in
control.

It doesn't mean no libraries - feel free to import any external code (hopefully
well written and tested) to build the application.

Put simply, "Libraries: you call them. Frameworks: they call you."

This may be a controvertial definition. Please see [this Stack Overflow
question][frameworkLibrarySO] if you want more nuance and better links

As there is no JavaScript, there will be naturally be no front-end frameworks or
libraries (React, Angular _et al._).

### The same API

Each implementation should conform to the same API as outlined in the acceptance test.

### Identical, accessible HTML 5

Firstly, every implementation should render the same HTML.

Secondly, that HTML should be accessible.

Thirdly, that HTML is HTML 5.

_How_ that HTML is rendered is entirely down to the application - handlebars,
React templates, some big html library. Just as long as it is the same HTML.

### Plain CSS 3

Plain, readable CSS 3 should be used pretty-up the presentation. That's not to
say that you can't be adventurous, or artistic, or amazing - take a look at
what's possidle in the [CSS Zen Garden][CSSZG]. Only that we don't want any CSS
pre-processors constructing that CSS.

And as the HTML is always the same, then the same CSS should work on every
version of the application. Which is a nice bonus.

## Acceptance Testing

There is an acceptance test which asserts on the HTML and the API of the
application. See the [Readme][acceptanceReadme] for details.

## Contributing

Please see the [CONTRIBUTING.md][contributing] documentation for details
about how you can help.

## Prior Art

This started with an [angry blog post on Dev.To][rant].

This project takes its sublect matter from [TodoMVC][TodoMVC], a project which
shows off how to build a simple Todo application using client side technologies
(such as AngularJS, React and Vue.js).

It was inspired by:
  - Adrian Holovaty's dotJS talk [A Framework Author's Case Against Frameworks][Holovaty]
  - Chris James's [The Web I Want][CJWeb]

It is nothing to do with the Android architecture MVP.

## Wall of Inspirational Quotes

> It is up to developers to make their sites work (or at least fallback gracefully) whether JavaScript is or isn't available. The reason for JavaScript being unavailable, whether it's user choice, network conditions or browser interventions, is unimportant. Building a resilient experience is the only way we can serve our users best.

-- [Phil Nash on Dev.to](https://dev.to/philnash/comment/5688)

[TodoMVC]: http://todomvc.com/
[CSS3]: https://developer.mozilla.org/en-US/docs/Web/CSS/CSS3
[HTML5]: https://developer.mozilla.org/en-US/docs/Web/Guide/HTML/HTML5
[Holovaty]: https://www.youtube.com/watch?v=VvOsegaN9Wk
[CJWeb]: https://dev.to/quii/the-web-i-want-43o
[SimpleEasy]: https://github.com/matthiasn/talk-transcripts/blob/master/Hickey_Rich/SimpleMadeEasy.md
[rant]: https://dev.to/gypsydave5/why-you-shouldnt-use-a-web-framework-3g24
[frameworkLibrarySO]: https://stackoverflow.com/questions/148747/what-is-the-difference-between-a-framework-and-a-library
[CSSZG]: http://www.csszengarden.com/
[MVP]: https://en.wikipedia.org/wiki/Minimum_viable_product
[MVPlayer]: https://en.wikipedia.org/wiki/Most_valuable_player
[contributing]: ./CONTRIBUTING.md
