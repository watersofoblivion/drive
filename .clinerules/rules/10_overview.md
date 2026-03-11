# Overview

This is an app people can use to improve their on-track performance driving.

It takes data from various automotive data loggers focused on motorsports, analyzes the data, and then uses the data to help users understand the data and take away actionable insights.  There are a lot of similar products out there, but this project has a few features that make it different:

* **History and Cross-Cutting Insights**: The app doesn't look at each lap or session in isolation like a lot of tools.  And the tools that do allow comparisons allow only, for example, side-by-side comparisons of individual laps.  This app takes a different approach by analyzing your data across time front and center.  In essence: one lap doesn't matter; what matters is how consistent you are and how you are improving.
* **Mental Model**: The app uses a distinct mental model for performance driving.  The core concept is that of the "Nadir" of a turn.  The nadir is the point of the turn where several things happen at once: it's the lowest speed, most lateral G-force, highest yaw, highest rate of turn, and the point where you stop braking and start accelerating.  The software focuses on helping users drive in such a way that those things do in fact all happen at once, that they happen consistently lap after lap, and that over time they slowly progress to a faster and faster lap.
* **Visualization**: The app also takes a novel approach to data visualization.  Most apps either overlay telemetry over videos or display charts, graphs, and tables of the data collected by devices.  The app instead renders a 3D model of your vehicle driving the lap, with all the relevant data, charts, and graphs embedded in the 3D view.  The perspective is more akin to a chase camera in a video game than traditional driving applications, and it allows you to see the data in-situ with all the context that entails.

Later sections will cover these product goals and features and more in detail.