<template>
    <div class="container mx-auto mt-4 space-y-6">
        <!-- Theme Selection -->
        <div class="card bg-base-200 shadow-md border border-base-300 p-6">
            <h2 class="card-title flex items-center gap-2 mb-4">
                <SunMoon class="w-5 h-5 text-primary" />
                {{ t("theme.select_theme") }}
            </h2>
            <p class="text-base-content/70 mb-4">
                {{ t("theme.description") }}
            </p>

            <div class="flex flex-col gap-4">
                <button
                    v-for="theme in themes"
                    :key="theme"
                    @click="changeTheme(theme)"
                    class="btn border flex items-center justify-between px-6 py-3"
                    :class="{
                        'border-primary/50 bg-primary/10':
                            selectedTheme === theme,
                        'border-base-content/10': selectedTheme !== theme,
                    }"
                >
                    <div class="flex items-center gap-2">
                        <Sun
                            v-if="theme === 'light'"
                            class="w-5 h-5 text-amber-400"
                        />
                        <Moon v-else class="w-5 h-5 text-indigo-400" />
                        <span class="font-medium capitalize">{{
                            t(`theme.${theme}`)
                        }}</span>
                    </div>
                    <div
                        v-if="selectedTheme === theme"
                        class="badge badge-primary"
                    >
                        {{ t("theme.selected") }}
                    </div>
                </button>
            </div>
        </div>

        <!-- Background Image -->
        <div class="card bg-base-200 shadow-md border border-base-300 p-6">
            <h2 class="card-title flex items-center gap-2 mb-4">
                <Image class="w-5 h-5 text-primary" />
                {{ t("customization.background_title") }}
            </h2>

            <div class="space-y-6">
                <div>
                    <label class="label text-sm font-medium text-base-content">
                        {{ t("customization.background_image") }}
                    </label>
                    <div class="relative">
                        <input
                            type="text"
                            class="input input-bordered w-full pr-10"
                            :value="backgroundImage"
                            :placeholder="t('customization.background_image_placeholder')"
                            @input="handleBackgroundInput('backgroundImage', ($event.target as HTMLInputElement).value)"
                        />
                        <button
                            v-if="backgroundImage"
                            class="absolute right-2 top-1/2 -translate-y-1/2 btn btn-xs btn-ghost"
                            @click="handleBackgroundInput('backgroundImage', '')"
                        >
                            &times;
                        </button>
                    </div>
                    <p class="text-xs text-base-content/50 mt-1">
                        {{ t("customization.background_image_help") }}
                    </p>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <div>
                        <div class="flex justify-between mb-2">
                            <label class="text-sm font-medium text-base-content">
                                {{ t("customization.background_blur") }}
                            </label>
                            <span class="text-xs font-mono">{{ backgroundBlur ?? 0 }}px</span>
                        </div>
                        <input
                            type="range"
                            min="0"
                            max="20"
                            step="1"
                            class="range range-primary range-sm"
                            :value="backgroundBlur ?? 0"
                            @input="handleBackgroundInput('backgroundBlur', Number(($event.target as HTMLInputElement).value))"
                        />
                    </div>

                    <div>
                        <div class="flex justify-between mb-2">
                            <label class="text-sm font-medium text-base-content">
                                {{ t("customization.background_opacity") }}
                            </label>
                            <span class="text-xs font-mono">{{ backgroundOpacity ?? 100 }}%</span>
                        </div>
                        <input
                            type="range"
                            min="0"
                            max="100"
                            step="5"
                            class="range range-primary range-sm"
                            :value="backgroundOpacity ?? 100"
                            @input="handleBackgroundInput('backgroundOpacity', Number(($event.target as HTMLInputElement).value))"
                        />
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import { SunMoon, Moon, Sun, Image } from "lucide-vue-next";
import { useToast } from "../services/toastService";
import { themeService } from "../services/themeService";

const { t } = useI18n();
const { addToast } = useToast();

const themes = ["light", "dark"];
const selectedTheme = ref("dark");
const backgroundImage = ref("");
const backgroundBlur = ref(0);
const backgroundOpacity = ref(100);

const changeTheme = async (theme: string) => {
    try {
        selectedTheme.value = theme;
        await themeService.setTheme(theme);
        addToast(t("theme.changed_success"), "success");
    } catch (error) {
        console.error("Failed to change theme:", error);
        addToast(t("theme.changed_failed"), "error");
    }
};

const handleBackgroundInput = (key: string, value: string | number) => {
    if (key === "backgroundImage") {
        backgroundImage.value = value as string;
        themeService.setBackgroundImage(value as string);
    } else if (key === "backgroundBlur") {
        backgroundBlur.value = value as number;
        themeService.setBackgroundBlur(value as number);
    } else if (key === "backgroundOpacity") {
        backgroundOpacity.value = value as number;
        themeService.setBackgroundOpacity(value as number);
    }
};

onMounted(async () => {
    selectedTheme.value = themeService.getCurrentTheme();
    backgroundImage.value = themeService.getBackgroundImage();
    backgroundBlur.value = themeService.getBackgroundBlur();
    backgroundOpacity.value = themeService.getBackgroundOpacity();
});
</script>
