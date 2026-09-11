# A note on how this gets built

I use AI assistance — Claude, specifically — to help build and maintain this
project, and you'll see it in the commit history. I'm not going to pretend
otherwise, and I'm not going to bury it either.

Here's what that actually means:

## What's mine

The idea for cyberplug, the decision to make it a terminal-first plugin
manager instead of another GUI settings panel, the keybinding scheme, the
screen layout, the whole "cyberdeck" aesthetic this and every other tool in
my stack shares — that's mine. I spent 30+ years as an auto tech before I
came to this, and the one thing that trade teaches you cold is that the tool
in your hand doesn't make the diagnosis. You do. I architect these projects,
I decide what they should look and feel like, I decide what's worth building
and what's scope creep, and I decide when something's actually done versus
just compiling.

I also do the thing most people skip: I run this stuff for real, on my own
machine, in my own workflow, before I ever call it finished. Bugs like the
Discover tabs silently vanishing on a cold cache don't get caught by reading
code — they get caught by using it and noticing something's off. That part's
on me every time.

## What the AI does

Grunt work, mostly. Boilerplate, refactors, hunting down exactly why a
blocking network call in a key handler was freezing a redraw, hardening a
launcher script against patterns a security scanner will flag, writing docs
that actually match what the code does instead of what I meant to build
last month. It's a second set of eyes that doesn't get tired at 1am and
doesn't take it personally when I tell it the first draft is wrong.

I don't ship what I don't understand. If I can't explain why a fix works,
I'm not done reviewing it yet.

## Why say any of this

Because I'd rather you know going in than find out later and wonder what
else wasn't said. I spent most of my working life doing things the hard way
before tools like this existed. I don't have anything to prove by pretending
I still do it that way — working smart isn't a shortcut, it's the point.

Judge the code. That's always been the right way to do it anyway.

— darkstardevx
