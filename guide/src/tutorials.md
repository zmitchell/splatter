# Tutorials

In the previous chapter we prepared everything needed to start our own splatter
project! In this chapter, we will take a more focused look at all of the
different features we can add to our splatter project.

If you are new to splatter or Rust we recommend starting with the "Basics"
tutorials before going on. If you are feeling more confident, feel free to
choose your own adventure through the following tutorials depending on what you
want to add to your project!


## Rust Basics

Tutorials for learning the basics of Rust with splatter. 

- [Rust variables](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- [Rust control flows](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- [Rust functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)

## splatter Basics

A suite of tutorials for getting familiar with the splatter environment.

- [Anatomy of a splatter app](/tutorials/basics/anatomy-of-a-splatter-app.md)
- [Drawing a sketch](/tutorials/basics/draw-a-sketch.md)
- [Sketch vs App](/tutorials/basics/sketch-vs-app.md)
- [Window Coordinates](/tutorials/basics/window-coordinates.md)
- splatter events

## Drawing

Working with splatter's `Draw` API - a simple approach of coding graphics.

- [Drawing 2D shapes](/tutorials/draw/drawing-2d-shapes.md)
- [Animating a circle](/tutorials/draw/animating-a-circle.md)
- [Drawing images](/tutorials/draw/drawing-images.md)
- Drawing 3D shapes
- Drawing meshes

## Windowing

Walk-throughs for creating and working with one or more windows.

- Building a custom window
- Creating multiple windows
- Drawing to different windows
- Fullscreen on startup
- Automatically positioning windows

## GUI

How to create a GUI (Graphical User Interface) for you splatter project.

*NOTE: It might be best to wait for
[#383](https://github.com/splatter-org/splatter/issues/383) before addressing
these.*

- Creating a UI
- Exploring available UI widgets
- Multi-window UI

## Audio

A suite of guides for working with audio in splatter.

- Setting up audio output
- Setting up audio input
- Selecting specific audio devices
- Playing an audio file
- Basic audio synthesis
- Many channel audio streams
- Feeding audio input to output
- Visualising audio

## Video

Loading, playing and recording video in splatter.

- Drawing video
- Recording a window to video
- Manipulating video playback

## WGPU

Understanding the lower level that underlies all graphics in splatter.

- What is WGPU?
- The graphics pipeline
- Compute shaders
- Fragment shaders
- Vertex shaders

## Projection Mapping

Getting started with using splatter for projection mapping.

- Basic quad-warping.

## LASERs

Detecting and outputting to LASER DACs on a network.

- Connecting to a LASER.
- Detecting LASER DACs.
- Tweaking LASER interpolation and optimisations.

## OSC

- [An intro to OSC](/tutorials/osc/osc-introduction.md).
- [Sending OSC](/tutorials/osc/osc-sender.md).
- Receiving OSC.

## DMX

Working with DMX over the network via sACN.

- Working with the sacn crate.

## Serial over USB

Working with Serial data in a cross-platform manner.

- Reading USB serial data.
- Writing USB serial data.

<br>

---

If you were unable to find what you were looking for above, or if you have an
idea for a tutorial not yet present, please feel free to create an issue or a
pull request!
