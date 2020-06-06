# ic_layout_db
A standalone database useful for constructing and manipulating integrated circuit layouts.

# Overall Architecture
* Initially the plan is to follow an ECS style architecture for the database components.
* The API for managing aspects of the design will follow similar ideas as gdspy. There will be a notion of a
db which holds all current "cells", and cells can hold shapes/labels/metadata to be drawn.
* The intent is to be able to attach metadata to any arbitrary component in the system for maximum flexibility.

# Tasks
- [ ] Create a point struct which will be used everywhere
- [x] Create a basic rectangle struct and methods to create/query it
- [ ] Create a cell struct, which will hold references to all the rects that it will contain, and provides methods
for constructing and manipulating shapes.
- [ ] Create a db struct, which will actually own all of the cells and data.
