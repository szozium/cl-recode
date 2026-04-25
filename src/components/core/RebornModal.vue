<script setup lang="ts">
import { openUrl } from "@tauri-apps/plugin-opener";

const props = defineProps<{ show: boolean }>();
const emit = defineEmits<{
    (event: "update:show", value: boolean): void;
}>();

const close = () => {
    emit("update:show", false);
};

const openGithub = async () => {
    try {
        await openUrl("https://github.com/woggc");
    } catch (error) {
        console.error("Failed to open GitHub:", error);
    }
};
</script>

<template>
    <div
        v-if="props.show"
        class="fixed inset-0 z-[9999] flex items-center justify-center bg-gradient-to-br from-black/80 via-black/70 to-black/80 backdrop-blur-xl p-6"
        @click.self="close"
    >
        <div
            class="relative max-w-3xl w-full max-h-[90vh] overflow-y-auto rounded-2xl bg-gradient-to-br from-base-300/90 via-base-200/95 to-base-300/90 backdrop-blur-md border border-primary/30 shadow-2xl animate-modal-enter"
        >
            <!-- Animated background gradient -->
            <div class="absolute inset-0 bg-gradient-to-br from-primary/5 via-transparent to-accent/5 animate-gradient"></div>

            <!-- Glow effect -->
            <div class="absolute -top-24 -right-24 w-48 h-48 bg-primary/20 rounded-full blur-3xl animate-pulse-slow"></div>
            <div class="absolute -bottom-24 -left-24 w-48 h-48 bg-accent/20 rounded-full blur-3xl animate-pulse-slow" style="animation-delay: 1s;"></div>

            <div class="relative p-10">
                <div class="flex flex-col items-center text-center gap-8">
                    <!-- Logo section -->
                    <div class="relative">
                        <div class="absolute inset-0 bg-primary/30 blur-2xl rounded-full animate-pulse"></div>
                        <div class="relative w-28 h-28 rounded-2xl bg-gradient-to-br from-primary/20 to-accent/20 backdrop-blur-sm border border-primary/40 flex items-center justify-center shadow-xl">
                            <img src="../../assets/images/logo.svg" class="w-16 h-16 drop-shadow-2xl" alt="Logo" />
                        </div>
                    </div>

                    <!-- Title section -->
                    <div class="space-y-4">
                        <div class="inline-block px-4 py-1.5 rounded-full bg-primary/10 border border-primary/30 backdrop-blur-sm">
                            <span class="text-xs font-bold tracking-widest uppercase text-primary">Codename: Rework</span>
                        </div>

                        <h1 class="text-5xl font-black tracking-tight">
                            <span class="bg-gradient-to-r from-base-content via-primary to-base-content bg-clip-text text-transparent">
                                CollapseLoader
                            </span>
                            <br />
                            <span class="text-primary drop-shadow-lg">Rework</span>
                        </h1>

                        <div class="flex items-center justify-center gap-2">
                            <div class="h-px w-12 bg-gradient-to-r from-transparent to-primary/50"></div>
                            <div class="w-2 h-2 rounded-full bg-primary animate-pulse"></div>
                            <div class="h-px w-12 bg-gradient-to-l from-transparent to-primary/50"></div>
                        </div>
                    </div>

                    <!-- Description -->
                    <div class="max-w-lg space-y-3">
                        <p class="text-lg text-base-content/90 leading-relaxed">
                            возобновление проекта с новыми фичами,
                            улучшенной производительностью и современным интерфейсом.
                        </p>
                        <div class="flex items-center justify-center gap-2 text-sm text-base-content/60">
                            <span>Разработка:</span>
                            <span class="font-mono font-bold text-primary">woggc</span>
                        </div>
                    </div>

                    <!-- Action buttons -->
                    <div class="flex flex-col sm:flex-row gap-3 w-full max-w-md mt-4">
                        <button
                            @click="openGithub"
                            class="btn btn-outline btn-primary flex-1 gap-2 group hover:scale-105 transition-transform"
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="group-hover:rotate-12 transition-transform">
                                <path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4"></path>
                                <path d="M9 18c-4.51 2-5-2-7-2"></path>
                            </svg>
                            <span class="font-semibold">@woggc</span>
                        </button>
                        <button
                            @click="close"
                            class="btn btn-primary flex-1 gap-2 hover:scale-105 transition-transform shadow-lg"
                        >
                            <span class="font-semibold">Начать работу</span>
                            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <path d="M5 12h14"></path>
                                <path d="m12 5 7 7-7 7"></path>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
@keyframes modal-enter {
    from {
        opacity: 0;
        transform: scale(0.95) translateY(20px);
    }
    to {
        opacity: 1;
        transform: scale(1) translateY(0);
    }
}

@keyframes gradient {
    0%, 100% {
        opacity: 1;
    }
    50% {
        opacity: 0.8;
    }
}

@keyframes pulse-slow {
    0%, 100% {
        opacity: 0.3;
        transform: scale(1);
    }
    50% {
        opacity: 0.5;
        transform: scale(1.1);
    }
}

.animate-modal-enter {
    animation: modal-enter 0.4s cubic-bezier(0.34, 1.56, 0.64, 1) forwards;
}

.animate-gradient {
    animation: gradient 3s ease-in-out infinite;
}

.animate-pulse-slow {
    animation: pulse-slow 4s ease-in-out infinite;
}
</style>
