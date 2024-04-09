import { createRouter,createWebHashHistory } from "vue-router";
import One from "./components/One.vue"
import Two from "./components/Two.vue"
import Nindex from "./components/Nindex.vue"


const router  = createRouter({
    history:createWebHashHistory(),
    routes:[
        {
            path:"/",
            component:Nindex
        },
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