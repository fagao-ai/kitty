export const routes = [
    {
        path: "/",
        name: "proxy",
        component: () => import("@/views/proxy/Index.vue"),
    },
]