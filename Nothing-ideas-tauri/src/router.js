import { createRouter,createWebHashHistory } from "vue-router";
import Detail from "./components/Detail.vue"
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
            path:"/Detail",
            component:Detail
        },
        {
            path:"/Two",
            component:Two
        }
    ]

})


export default router;