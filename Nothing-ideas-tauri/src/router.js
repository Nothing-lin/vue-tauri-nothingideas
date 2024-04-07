import { createRouter,createWebHashHistory } from "vue-router";
import One from "./components/One.vue"
import Two from "./components/Two.vue"

const router  = createRouter({
    history:createWebHashHistory(),
    routes:[
        {
            path:"/One",
            component:One
        },
        {
            path:"/Two",
            component:Two
        }
    ]

})


export default router;