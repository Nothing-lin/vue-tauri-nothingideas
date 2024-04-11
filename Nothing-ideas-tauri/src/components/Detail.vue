<template>
  <div class="common-layout">
    <el-container>
      <!-- Header -->
      <el-header
        style="background-color: #a0cfff;padding: 0%;display: flex;justify-content: space-between; align-items: center;padding: 20px;">
        <div style="display: flex;align-items: center;">
          <el-button :icon="Back" style="box-shadow: none;" @click="$router.push('/')">返回</el-button>
          <h1 style="margin-left: 20px; color: #337ecc;font-size: 1em;">如何通过tauri开发出一款自定义的个人app</h1>
        </div>

        <el-button-group class="btn-group">
          <el-button type="primary" size="mini" plain style="box-shadow: none;"
            @click="dialogFormVisible = true">新增节点</el-button>
          <el-button type="success" size="mini" plain style="box-shadow: none;">流程闭环</el-button>
          <el-button type="warning" size="mini" plain style="box-shadow: none;">恢复流程</el-button>
          <el-button size="mini" plain style="box-shadow: none;" @click="fetchPreojectNodes()">测试按钮{{ project_id
          }}</el-button>
        </el-button-group>
      </el-header>
      <!-- Main -->
      <el-main style="width: 100%;margin: auto;">
        <div class="timeLine-container">
          <el-container>
            <el-main class="timeline-list-container">
              <!-- 时间轴 -->
              <div class="block">
                <el-timeline>
                  <!-- node节点列表 -->
                  <el-timeline-item timestamp="2018年4月12日" placement="top" v-for="item in NothingProjectNodes">
                    <template v-slot:dot>
                      <!-- 使用 dot 插槽自定义小圆点 -->
                      <span class="custom-dot" :style="{ backgroundColor: item.node_type === '拓展' ? 'red' : 'none' }">{{
                        item.node_type }}</span>
                    </template>
                    <el-card class="custom-card">
                      <el-row :gutter="20">
                        <el-col :span="16">
                          <div class="grid-content bg-purple">
                            <h3 style="margin-top: 0px;">{{ item.node_title }}</h3>
                            <p>{{ item.node_text }}</p>
                          </div>
                        </el-col>
                        <el-col :span="8">
                          <div class="grid-content bg-purple">
                            <h4>参考网站</h4>
                            <ul>
                              <li><el-link type="success">参考链接1</el-link></li>
                              <li><el-link type="success">参考链接2</el-link></li>
                              <li><el-link type="success">参考链接3</el-link></li>
                            </ul>
                          </div>
                        </el-col>
                      </el-row>
                    </el-card>
                  </el-timeline-item>
                </el-timeline>
              </div>

            </el-main>
          </el-container>
        </div>
      </el-main>

      <!-- 新增-对话框 -->
      <el-dialog v-model="dialogFormVisible" title="新增节点" width="800">
        <el-form :model="form">
          <div style="display: flex;align-items: center;margin-bottom: 20px;">
            <span style="width: 100px">节点标题：</span>
            <el-input v-model="form.name" autocomplete="off" />
          </div>

          <div style="display: flex;align-items: center;margin-bottom: 20px;">
            <span style="width: 100px">节点类型：</span>
            <el-select v-model="type_value" placeholder="Select" style="width: 240px;left: 0px;">
              <el-option v-for="item in options" :key="item.value" :label="item.label" :value="item.value" />
            </el-select>
          </div>

          <div style="display: flex;align-items: center;margin-bottom: 20px;">
            <span style="width: 100px;margin-bottom: 10px;">节点标题：</span>
            <el-input v-model="Nodetextarea" style="width: 100%;margin-top: 10px;" autosize type="textarea"
              placeholder="Please input" />
          </div>


        </el-form>
        <template #footer>
          <div class="dialog-footer">
            <el-button @click="dialogFormVisible = false">取消</el-button>
            <el-button type="primary" @click="addNode()">
              新增
            </el-button>
          </div>
        </template>
      </el-dialog>
    </el-container>
  </div>
</template>


<script>
import Database from "tauri-plugin-sql-api";
import { Back, Edit, Search, Share, Upload } from '@element-plus/icons-vue'
import { reactive, ref } from 'vue'

export default {
  data() {
    return {
      project_id: null,
      NothingProjectNodes: []
    }
  },
  async created() {
    this.fetchPreojectNodes();
  },
  mounted() {
    this.project_id = this.$route.params.project_id//获取路由参数
    console.log(this.project_id)
  },
  methods: {
    async fetchPreojectNodes() {
      //加载数据库数据
      const db = await Database.load("sqlite:NothingIdeas.db")
      const query = `SELECT * FROM nothing_project_node WHERE project_id = ${this.project_id}`
      const NothingProjectNodes = await db.select(query);
      this.NothingProjectNodes = NothingProjectNodes;
      await db.close();
      console.log(this.NothingProjectNodes)
    },
    // 新增按钮插入数据
    async addNode() {
      const db = await Database.load("sqlite:NothingIdeas.db")
      const query = `INSERT INTO nothing_project_node (project_id, node_title, node_text, node_type,node_create_time) VALUES (?, ?, ?, ?,?)`
      const params = [this.project_id, this.form.name, this.Nodetextarea, this.type_value, new Date().toLocaleString()]
      const result = await db.execute(query, params);
      console.log(result)
      await db.close();
      this.fetchPreojectNodes();
      this.dialogFormVisible = false;
      this.form.name = ''
      this.Nodetextarea = ''
      this.type_value = ''
    }
  },
  setup() {

    // 新增按钮-对话框
    const dialogFormVisible = ref(false)
    const formLabelWidth = '140px'

    const Nodetextarea = ref('')

    const form = reactive({
      name: '',
      region: '',
      date1: '',
      date2: '',
      delivery: false,
      type: [],
      resource: '',
      desc: '',
    })
    const value = ref('')

    const options = [
      {
        value: '主题',
        label: '主题',
      },
      {
        value: '拓展',
        label: '拓展',
      }
    ]

    return {
      dialogFormVisible,
      formLabelWidth,
      form,
      Nodetextarea,
      options,
      value
    };


  }

}
</script>


<style>
.el-card h4 {
  margin: 0% !important;
}

.el-timeline-item__wrapper {
  padding-left: 75px !important;
  top: 5px !important;
}

span.custom-dot {
  color: white;
  background-color: rgb(66, 164, 255);
  padding-top: 5px;
  padding-bottom: 5px;
  padding-left: 15px;
  padding-right: 15px;
  border-radius: 5px;
  margin-left: -25px;
}

.timeline-list-container {
  width: 60%;
  margin: auto;

}

.header-button-container {
  margin-top: 10px;
  margin-left: auto;
  float: inline-end;
}

.backpage-icon {
  margin-top: 15px !important;
}

.header-container h2 {
  color: white;
  font-size: larger;
  margin-left: -40px;
  overflow: auto;
}

.header-container {
  background-color: #1abc9c;
  height: 100px !important;
  padding: 20px !important;
}

.timeLine-container {
  width: 100%;
  height: 100vh;
  margin: -10px;
  padding: 0;
  overflow: auto;
}

input,
button {
  box-shadow: none !important;
}
</style>
