# An audio app for samples pipilining

The application allows the user to manipulate sounds by concatenation geometrical shapes with basic
audio operations.

There are three basics sounds:
* DOT
* DASH
* PAUSE

There are three basic shapes:
* square
* triangle
* pentagon

Each of them is assigned a sound pattern.

Shapes can be concatenated in shapes-operations patterns to form complex samples chains.


## Try it

* `cargo run`
* select two shapes
* select an operation
* listen to the result

## WIP

This should be the initial set up for an mini audio-engine to build a Web-app on top, with a proper UI and
UX to generate sounds from samples.
The frontend may be in [Gate](https://github.com/SergiusIW/gate/tree/master/) or by leveraging
[Yew](https://github.com/DenisKolodin/yew) and [WebAudio API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Audio_API/Basic_concepts_behind_Web_Audio_API).