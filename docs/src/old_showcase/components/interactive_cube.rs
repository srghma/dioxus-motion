use dioxus::prelude::*;
use dioxus_motion::prelude::*;

const CONTAINER_SIZE: f32 = 200.0; // Increased size for better visibility
const PERSPECTIVE: f32 = 800.0; // Increased perspective for more dramatic 3D effect

#[component]
pub fn InteractiveCube() -> Element {
    let mut rotation_x = use_motion(0.0f32);
    let mut rotation_y = use_motion(0.0f32);
    let mut rotation_z = use_motion(0.0f32); // Added Z rotation for more dynamics
    let mut scale = use_motion(1.0f32);
    let mut glow = use_motion(0.2f32); // Initial subtle glow
    let mut hover_lift = use_motion(0.0f32); // New hover effect

    let onclick = move |_e: Event<MouseData>| {
        // Enhanced spin animation
        let spin_sequence = AnimationSequence::new().then(
            rotation_y.get_value() + 360.0,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 150.0,
                damping: 12.0,
                mass: 1.0,
                velocity: 25.0,
            })),
        );

        // Enhanced bounce animation
        let bounce_sequence = AnimationSequence::new()
            .then(
                1.3, // Bigger bounce
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 400.0,
                    damping: 8.0,
                    mass: 1.0,
                    velocity: 8.0,
                })),
            )
            .then(
                1.0,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 300.0,
                    damping: 15.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            );

        // Z-axis wobble effect
        let wobble_sequence = AnimationSequence::new()
            .then(
                15.0,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 200.0,
                    damping: 5.0,
                    mass: 0.5,
                    velocity: 10.0,
                })),
            )
            .then(
                0.0,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 200.0,
                    damping: 10.0,
                    mass: 0.5,
                    velocity: 0.0,
                })),
            );

        scale.animate_sequence(bounce_sequence);
        rotation_y.animate_sequence(spin_sequence);
        rotation_z.animate_sequence(wobble_sequence);

        // Enhanced glow effect
        glow.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 300.0,
                damping: 10.0,
                mass: 0.5,
                velocity: 5.0,
            })),
        );

        // Reset glow after animation
        glow.animate_to(
            0.2,
            AnimationConfig::new(AnimationMode::Spring(Spring::default()))
                .with_delay(std::time::Duration::from_millis(500)),
        );
    };

    let onmousemove = move |e: Event<MouseData>| {
        let rect = e.data().client_coordinates();
        let x = (rect.x as f32 - CONTAINER_SIZE / 2.0) / (CONTAINER_SIZE / 2.0);
        let y = (rect.y as f32 - CONTAINER_SIZE / 2.0) / (CONTAINER_SIZE / 2.0);

        // Smoother rotation response
        rotation_x.animate_to(
            -y * 30.0, // Inverted for natural movement
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 150.0,
                damping: 15.0,
                mass: 0.8,
                velocity: 0.0,
            })),
        );

        rotation_y.animate_to(
            x * 30.0,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 150.0,
                damping: 15.0,
                mass: 0.8,
                velocity: 0.0,
            })),
        );
    };

    let onmouseenter = move |_| {
        hover_lift.animate_to(
            20.0,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 200.0,
                damping: 15.0,
                mass: 0.8,
                velocity: 0.0,
            })),
        );
    };

    let onmouseleave = move |_| {
        hover_lift.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 200.0,
                damping: 15.0,
                mass: 0.8,
                velocity: 0.0,
            })),
        );

        // Reset rotations
        rotation_x.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 150.0,
                damping: 15.0,
                mass: 0.8,
                velocity: 0.0,
            })),
        );

        rotation_y.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 150.0,
                damping: 15.0,
                mass: 0.8,
                velocity: 0.0,
            })),
        );
    };

    rsx! {
        div {
            class: "relative cursor-pointer select-none",
            style: "width: {CONTAINER_SIZE}px; height: {CONTAINER_SIZE}px; perspective: {PERSPECTIVE}px",
            // Enhanced glow background
            div {
                class: "absolute inset-0 bg-linear-to-r from-blue-500/30 to-purple-500/30 blur-3xl -z-10 transition-all duration-300",
                style: "opacity: {glow.get_value()}; transform: scale({1.0 + glow.get_value() * 0.2})",
            }

            // Shadow
            div {
                class: "absolute bottom-0 left-1/2 -translate-x-1/2 bg-black/20 blur-xl rounded-full transition-all duration-300",
                style: "width: {CONTAINER_SIZE * 0.8}px; height: {CONTAINER_SIZE * 0.1}px; transform: translateY({20.0 + hover_lift.get_value()}px) scale({scale.get_value()}, 1.0)",
            }

            div {
                onclick,
                onmousemove,
                onmouseenter,
                onmouseleave,
                class: "relative w-full h-full items-center justify-center transform-style-3d transition-all duration-100",
                style: "transform: translateY(-{hover_lift.get_value()}px) rotateX({rotation_x.get_value()}deg) rotateY({rotation_y.get_value()}deg) rotateZ({rotation_z.get_value()}deg) scale({scale.get_value()})",
                // Front face with enhanced gradient
                div {
                    class: "absolute w-full h-full flex items-center justify-center text-2xl font-bold text-white bg-linear-to-br from-blue-500 to-blue-600 shadow-lg transform translate-z-[100px] opacity-90 hover:opacity-100 transition-all duration-300",
                    style: "box-shadow: 0 0 30px rgba(59, 130, 246, 0.5)",
                    "Front"
                }
                // Back face
                div {
                    class: "absolute w-full h-full flex items-center justify-center text-2xl font-bold text-white bg-linear-to-br from-purple-500 to-purple-600 shadow-lg transform -translate-z-[100px] rotate-y-180 opacity-90 hover:opacity-100 transition-all duration-300",
                    style: "box-shadow: 0 0 30px rgba(147, 51, 234, 0.5)",
                    "Back"
                }
                // Right face
                div {
                    class: "absolute w-full h-full flex items-center justify-center text-2xl font-bold text-white bg-linear-to-br from-pink-500 to-pink-600 shadow-lg transform translate-x-[100px] rotate-y-90 opacity-90 hover:opacity-100 transition-all duration-300",
                    style: "box-shadow: 0 0 30px rgba(236, 72, 153, 0.5)",
                    "Right"
                }
                // Left face
                div {
                    class: "absolute w-full h-full flex items-center justify-center text-2xl font-bold text-white bg-linear-to-br from-green-500 to-green-600 shadow-lg transform -translate-x-[100px] -rotate-y-90 opacity-90 hover:opacity-100 transition-all duration-300",
                    style: "box-shadow: 0 0 30px rgba(34, 197, 94, 0.5)",
                    "Left"
                }
                // Top face
                div {
                    class: "absolute w-full h-full flex items-center justify-center text-2xl font-bold text-white bg-linear-to-br from-yellow-400 to-yellow-500 shadow-lg transform translate-y-[-100px] rotate-x-90 opacity-90 hover:opacity-100 transition-all duration-300",
                    style: "box-shadow: 0 0 30px rgba(234, 179, 8, 0.5)",
                    "Top"
                }
                // Bottom face
                div {
                    class: "absolute w-full h-full flex items-center justify-center text-2xl font-bold text-white bg-linear-to-br from-red-500 to-red-600 shadow-lg transform translate-y-[100px] -rotate-x-90 opacity-90 hover:opacity-100 transition-all duration-300",
                    style: "box-shadow: 0 0 30px rgba(239, 68, 68, 0.5)",
                    "Bottom"
                }
            }
        }
    }
}
