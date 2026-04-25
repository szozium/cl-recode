<script setup lang="ts">
import { ref } from "vue";
import { X, CheckCircle2, Circle, AlertCircle } from "lucide-vue-next";

const props = defineProps<{ show: boolean }>();
const emit = defineEmits<{
    (event: "update:show", value: boolean): void;
}>();

interface TodoItem {
    id: string;
    title: string;
    description: string;
    status: "pending" | "in-progress" | "completed";
    priority: "low" | "medium" | "high";
}

const todos = ref<TodoItem[]>([
    {
        id: "1",
        title: "Загрузить Forge на GitHub",
        description: "Файл forge-1.16.5-36.2.39-installer.jar (7.2MB) готов. Создать release с тегом 'forge' на https://github.com/szozium/cl-recode/releases/new и загрузить файл. После загрузки приложение будет качать Forge с GitHub mirror.",
        status: "completed",
        priority: "high"
    }
]);

const close = () => {
    emit("update:show", false);
};

const toggleStatus = (id: string) => {
    const todo = todos.value.find(t => t.id === id);
    if (!todo) return;

    if (todo.status === "pending") todo.status = "in-progress";
    else if (todo.status === "in-progress") todo.status = "completed";
    else todo.status = "pending";
};

const getPriorityColor = (priority: string) => {
    switch (priority) {
        case "high": return "text-error";
        case "medium": return "text-warning";
        case "low": return "text-info";
        default: return "text-base-content";
    }
};

const getStatusIcon = (status: string) => {
    switch (status) {
        case "completed": return CheckCircle2;
        case "in-progress": return AlertCircle;
        default: return Circle;
    }
};
</script>

<template>
    <div
        v-if="props.show"
        class="fixed inset-0 z-[9999] flex items-center justify-center bg-black/60 backdrop-blur-sm p-6"
        @click.self="close"
    >
        <div
            class="relative max-w-4xl w-full max-h-[85vh] overflow-y-auto rounded-2xl bg-base-200/95 backdrop-blur-md border border-primary/20 shadow-2xl"
        >
            <!-- Header -->
            <div class="sticky top-0 z-10 bg-base-300/95 backdrop-blur-md border-b border-primary/20 p-6">
                <div class="flex items-center justify-between">
                    <div>
                        <h2 class="text-2xl font-black tracking-tight text-primary">
                            TODO / Development Tasks
                        </h2>
                        <p class="text-sm text-base-content/60 mt-1">
                            Активные задачи разработки
                        </p>
                    </div>
                    <button
                        @click="close"
                        class="btn btn-ghost btn-sm btn-circle hover:bg-error/20 hover:text-error transition-colors"
                        title="Close"
                    >
                        <X :size="18" />
                    </button>
                </div>
            </div>

            <!-- Todo List -->
            <div class="p-6 space-y-4">
                <div
                    v-for="todo in todos"
                    :key="todo.id"
                    class="bg-base-300/50 rounded-xl border border-base-content/10 p-5 hover:border-primary/30 transition-all"
                >
                    <div class="flex items-start gap-4">
                        <button
                            @click="toggleStatus(todo.id)"
                            class="mt-1 hover:scale-110 transition-transform"
                            :class="{
                                'text-success': todo.status === 'completed',
                                'text-warning': todo.status === 'in-progress',
                                'text-base-content/40': todo.status === 'pending'
                            }"
                        >
                            <component :is="getStatusIcon(todo.status)" :size="24" />
                        </button>

                        <div class="flex-1 min-w-0">
                            <div class="flex items-center gap-3 mb-2">
                                <h3
                                    class="text-lg font-bold"
                                    :class="{
                                        'line-through text-base-content/50': todo.status === 'completed'
                                    }"
                                >
                                    {{ todo.title }}
                                </h3>
                                <span
                                    class="badge badge-sm font-semibold uppercase tracking-wider"
                                    :class="getPriorityColor(todo.priority)"
                                >
                                    {{ todo.priority }}
                                </span>
                            </div>
                            <p
                                class="text-sm text-base-content/70 leading-relaxed"
                                :class="{
                                    'line-through text-base-content/40': todo.status === 'completed'
                                }"
                            >
                                {{ todo.description }}
                            </p>
                            <div class="mt-3 flex items-center gap-2">
                                <span
                                    class="text-xs font-semibold px-2 py-1 rounded-full"
                                    :class="{
                                        'bg-success/20 text-success': todo.status === 'completed',
                                        'bg-warning/20 text-warning': todo.status === 'in-progress',
                                        'bg-base-content/10 text-base-content/60': todo.status === 'pending'
                                    }"
                                >
                                    {{ todo.status === 'completed' ? 'Завершено' : todo.status === 'in-progress' ? 'В работе' : 'Ожидает' }}
                                </span>
                            </div>
                        </div>
                    </div>
                </div>

                <div v-if="todos.length === 0" class="text-center py-12">
                    <CheckCircle2 :size="48" class="mx-auto text-success/50 mb-4" />
                    <p class="text-base-content/60">Все задачи выполнены</p>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
@keyframes slideIn {
    from {
        opacity: 0;
        transform: scale(0.95) translateY(20px);
    }
    to {
        opacity: 1;
        transform: scale(1) translateY(0);
    }
}

.fixed > div {
    animation: slideIn 0.3s cubic-bezier(0.34, 1.56, 0.64, 1) forwards;
}
</style>
