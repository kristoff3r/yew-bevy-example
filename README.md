# yew-bevy-example

This is a small example illustrating how to run bevy and yew (or another frontend framework) with events and shared state between them.

Note that the shared state doesn't currently update consistently between the 2,
that requires synchronizing re-renders (on the yew side) and doing component
updates (on the bevy side) by using the events.
