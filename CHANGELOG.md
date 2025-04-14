# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0](https://github.com/wheregmis/dioxus-motion/compare/dioxus-motion-v0.3.1...dioxus-motion-v0.4.0) - 2025-04-14

### Fixed

- fixed page transition animation from/to to root route
- fixing animation config explicit conversion

### Other

- More Guide
- Update readme
- Using dioxus main branch
- Clippy FIxes
- Merge branch 'main' of https://github.com/wheregmis/dioxus-motion
- Modified and Added some tests
- Few Timing optimization for desktop
- Clippy Fixes
- Fix Nested Page transition
- Setting basepath for github deployment
- Remove example project and embed it into docs
- step calculation fix
- optimize animation state updates and route handling in effects
- remove AnimationSignal implementation from AnimationManager trait
- More cleanup
- few cleanup
- animation step stack allocation
- more spring optimizations
- optimized spring
- using arc for shared config, memory opt
- adaptive delay calculation
- 📝 Add docstrings to `ft_docs`
- Fix Clippy
- Revert page transitions and platform.rs
- Few opts
- Wrap first Draft on Dioxus Motion Docs
- Making docs cross platform
- Basic Landing Page
- Update changelog to use consistent versioning format
- Update changelog to include project documentation and semantic versioning details
- Update changelog
- Refactor dioxus-motion-transitions-macro package structure and remove obsolete route_transitions crate
- Update Cargo.toml to use resolver version 3
- Add release-plz configuration for dioxus-motion packages
- Update dependencies: syn to 2.0.100, quote to 1.0.40, and proc-macro2 to 1.0.94
- Disable publish.yml
- remove router_test from workspace
- Lets try release plz action
- Lock tokio to 1.43.0
- Now we fully support nested routing
- Comment Github Pages Action
- Bringing back all the Page Transitions- AI Generated
- Making version 0.3.1  so it wont publish things on crate.io
- Clean and Easy Animation For now
- wip nested route
- Thanks to Evan
- Using use_context_provider
- Update the changelog
- Using Outlet to Include Layout for the time being
- Dumping all the changes
- Not showing layout, now need to show it somehow
- Update workspace configuration and add utils module for transitions

## [0.1.1](https://github.com/wheregmis/dioxus-motion/compare/dioxus-motion-transitions-macro-v0.1.0...dioxus-motion-transitions-macro-v0.1.1) - 2025-04-14

### Other

- Refactor dioxus-motion-transitions-macro package structure and remove obsolete route_transitions crate
### Fixes:
- Layout not being shown when animating in the case of nested Layouts
- Nested Layout fully fixed
### Changes:
- Few code refactoring

## [0.3.1] - 2024-02-08
- Rerelease

## [0.3.0] - 2024-02-08
### New Features
- Added initial support for page transitions (Special thanks to Marc and Evan)
### Bug Fixes or Enhancements
- Support dioxus 0.6.3
### Changes
- Most of the things should be on the prelude, so if you face any erros while migrating, just import prelude::*.

## [0.2.3] - 2024-01-23
### Dioxus Version Bump
- updated to dioxus v0.6.2
- minor fixes

## [0.2.2] - 2024-01-17
### Performance Improvements
- Resource optimization for web

## [0.2.1] - 2024-01-11
### Performance Improvements
- Smoothness Optimization
### New Features
- Animation Sequence

## [0.2.0] - 2024-01-05
### Breaking Changes
- Replaced `use_value_animation` and `use_transform_animation` with `use_motion` hook
- Removed old animation configuration system
- Updated Transform property names for consistency
- Changed spring physics default parameters
- Removed deprecated animation methods

### New Features
- Added Color animation support
- Introduced new `AnimationConfig` API
- Added support for animation delays
- Implemented loop modes (Infinite, Times)
- Added new spring physics configuration
- Improved cross-platform performance
- Added new examples and documentation

### Performance Improvements
- Optimized animation frame handling
- Reduced CPU usage on desktop platforms
- Improved interpolation calculations
- Better memory management
- Enhanced cleanup on unmount

### Bug Fixes
- Fixed color interpolation for decreasing values
- Corrected spring physics calculations
- Fixed desktop platform timing issues
- Resolved memory leaks in animation loops
- Fixed transform rotation interpolation

## 🆕 What's New in v0.2.0

### New Animation API
- Unified animation hook `use_animation`
- Simplified configuration
- Enhanced type safety
- Better performance

### Color Animations
```rust
let color = use_motion(Color::from_rgba(59, 130, 246, 255));
color.animate_to(
    Color::from_rgba(168, 85, 247, 255),
    AnimationConfig::new(AnimationMode::Spring(Spring::default()))
);
```
### Animation Delays & Loops
```rust
AnimationConfig::new(mode)
    .with_delay(Duration::from_secs(1))
    .with_loop(LoopMode::Times(3))
```

## [0.1.4] - 2024-12-28
### Changes
- Update dependencies and remove unused UUID references
- Stop animations on component drop for improved resource management
- Refactor delay function to improve animation frame handling
- Optimize animation frame handling for smoother performance
- Add Screen feature to web-sys and improve frame time calculation
- Force target 90 FPS hardcoding for consistent performance

### Fixes
- Remove Tailwind CDN dependency from Index.html
- Remove Particle Effect temporarily for stability
- Revert to initial implementation of delay function
- Code cleanup and optimization

## [0.1.3] - 2024-12-27
### Changes
- Adjust animation frame threshold for smoother performance

### Fixes
- Fixed Desktop Platform (Seemed to be broken previously)

## [0.1.2] - 2024-12-27
### Changes
- Example Overhaul

### Fixes
- Fixed Desktop Platform (Seemed to be broken previously)

## [0.1.1] - 2024-12-27
### Changes
- Update Readme

## [0.1.0] - 2024-12-27
### Changes
- Initial Release