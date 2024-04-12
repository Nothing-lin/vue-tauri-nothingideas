<template>
  <div class="common-layout">
    <el-container>
      <!-- Header -->
      <el-header
        style="background-color: #a0cfff;padding: 0%;display: flex;justify-content: space-between; align-items: center;padding: 20px;">
        <div style="display: flex;align-items: center;">
          <el-button :icon="Back" style="box-shadow: none;" @click="$router.push('/')">返回</el-button>
          <h1 style="margin-left: 20px; color: #337ecc;font-size: 1em;">{{ this.projectname }}</h1>
        </div>

        <el-button-group class="btn-group">
          <el-button type="primary" size="mini" plain style="box-shadow: none;"
            @click="dialogFormVisible = true">新增节点</el-button>
          <el-button type="success" size="mini" plain style="box-shadow: none;border-color: none;"
            :disabled="this.projectstaus === '已闭环'" @click="updateProjectStatus()">流程闭环</el-button>
          <el-button type="warning" size="mini" plain style="box-shadow: none;border-color: none;"
            :disabled="this.projectstaus === '未闭环'" @Click="recoverProjectStatus()">恢复流程</el-button>
          <el-button size="mini" plain style="box-shadow: none;" @click="fetchPreoject()">测试按钮{{ project_id
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
                  <el-timeline-item :timestamp="formatDate(new Date(item.node_create_time))" placement="top" v-for="item in NothingProjectNodes"
                    :key="item.node_id">
                    <template v-slot:dot>
                      <!-- 使用 dot 插槽自定义小圆点 -->
                      <span class="custom-dot" :style="{ backgroundColor: item.node_type === '拓展' ? 'red' : 'none' }">{{
                        item.node_type }}</span>
                    </template>
                    <el-card class="custom-card">
                      <el-row :gutter="23">
                        <el-col :span="19">
                          <div class="grid-content bg-purple">
                            <h3 style="margin-top: 0px;border-color: #f6f6f657;border-width: 1px;border-bottom-style: outset;padding-bottom: 10px;color: #42a4ff;">{{ item.node_title }}</h3>
                            <!-- <p>{{ item.node_text }}</p> -->
                            <div v-html="item.node_text"></div>
                          </div>
                        </el-col>
                        <el-col :span="4">
                          <div class="grid-content bg-purple" v-if="this.projectstaus === '未闭环'">
                            <h4>操作</h4>
                            <ul>
                              <li><el-link type="primary"
                                  @click="getNodeData(item.node_type, item.node_title, item.node_text, item.node_id)">编辑</el-link>
                              </li>
                              <li><el-link type="danger" @click="deleteNode(item.node_id)">删除</el-link></li>
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
            <el-select v-model="type_value" placeholder="Select" style="width: 240px;">
              <el-option v-for="item in options" :key="item.value" :label="item.label" :value="item.value" />
            </el-select>
          </div>
          <!-- <QuillEditor theme="snow" /> -->
          <quill-editor ref="editor" v-model="editorContent" :options="editorOption"
            @text-change="handleTextChange"></quill-editor>

        </el-form>
        <template #footer>
          <div class="dialog-footer">
            <el-button @click="dialogFormVisible = false" @Click="clearForm()">取消</el-button>
            <el-button type="primary" @click="addNode()">
              新增
            </el-button>
          </div>
        </template>
      </el-dialog>


      <!-- 编辑-对话框 -->
      <el-dialog v-model="dialogFormEditVisible" title="编辑节点" width="800">
        <el-form :model="formEdit">
          <div style="display: flex;align-items: center;margin-bottom: 20px;">
            <span style="width: 100px">节点标题：</span>
            <el-input v-model="formEdit.name" autocomplete="off" />
          </div>

          <div style="display: flex;align-items: center;margin-bottom: 20px;">
            <span style="width: 100px">节点类型：</span>
            <el-select v-model="typeEdit_value" placeholder="Select" style="width: 240px;">
              <el-option v-for="item in options" :key="item.value" :label="item.label" :value="item.value" />
            </el-select>
          </div>
          <!-- <QuillEditor theme="snow" /> -->
          <quill-editor ref="Editeditor" v-model="editorContent2" :options="editorOption"
            @text-change="handleTextChange"></quill-editor>

        </el-form>
        <template #footer>
          <div class="dialog-footer">
            <el-button @click="dialogFormEditVisible = false" @Click="clearForm2()">取消</el-button>
            <el-button type="primary" @click="updateNode()">
              更新
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
import { QuillEditor } from '@vueup/vue-quill'
import '@vueup/vue-quill/dist/vue-quill.snow.css';

export default {
  components: {
    QuillEditor
  },
  data() {
    return {
      type_value: '',//节点类型,如果没声明那么显示不了
      editorContent: '',
      options: [],
      project_id: null,
      NothingProjectNodes: [],
      NothingProject: [],
      node_item: { 'node_type': '', 'node_title': '', 'node_text': '', 'node_id': '' },
      projectname: '',
      projectstaus: '',
      editorOption: {
        // 富文本编辑器配置选项
        placeholder: '请输入内容...', // 占位符
        modules: {
          toolbar: [
            [{ 'header': [1, 2, 3, 4, 5, 6, false] }],
            ['bold', 'italic', 'underline', 'strike'],
            ['blockquote', 'code-block'],
            [{ 'align': [] }], // 对齐方式
            [{ 'list': 'ordered' }, { 'list': 'bullet' }],
            [{ 'indent': '-1' }, { 'indent': '+1' }],
            ['link', 'image'],
            ['clean']
          ]
        }
      }
    }
  },
  async created() {
    await this.fetchPreojectNodes();
    this.NothingProject = await this.fetchPreoject();
    this.projectname = this.NothingProject[0].project_title;
    this.projectstaus = this.NothingProject[0].project_status;
    console.log(this.projectname)
  },
  async mounted() {
    this.project_id = this.$route.params.project_id//获取路由参数


  },
  methods: {
    // 加载数据库数据
    async fetchPreoject() {
      this.project_id = this.$route.params.project_id//获取路由参数
      console.log("我是标题")
      const db = await Database.load("sqlite:NothingIdeas.db")
      const query = `SELECT * FROM nothing_project WHERE project_id = ${this.project_id}`
      const project = await db.select(query);
      await db.close();
      return project;
    },
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
      const params = [this.project_id, this.form.name, this.$refs.editor.getHTML(), this.type_value, new Date().toLocaleString()]
      const result = await db.execute(query, params);
      console.log(result)
      await db.close();
      this.fetchPreojectNodes();
      this.dialogFormVisible = false;
      // 清空表单数据
      this.form.name = ''
      this.editorContent = ''
      this.type_value = ''
      this.$refs.editor.setHTML('')
      const content = this.editorContent;
      const content2 = this.$refs.editor.getHTML();
      console.log(content)
      console.log(content2)

    },
    handleTextChange(newContent, oldContent, source) {
      this.editorContent = newContent;
    },
    // 更新nothing_project表的project_status字段
    async updateProjectStatus() {
      const db = await Database.load("sqlite:NothingIdeas.db")
      const query = `UPDATE nothing_project SET project_status = '已闭环' WHERE project_id = ${this.project_id}`
      const result = await db.execute(query);
      console.log(result)
      await db.close();
      this.projectstaus = '已闭环'
    },
    // 恢复流程
    async recoverProjectStatus() {
      const db = await Database.load("sqlite:NothingIdeas.db")
      const query = `UPDATE nothing_project SET project_status = '未闭环' WHERE project_id = ${this.project_id}`
      const result = await db.execute(query);
      console.log(result)
      await db.close();
      this.projectstaus = '未闭环'
    },
    // 删除节点
    async deleteNode(nodeid) {
      const db = await Database.load("sqlite:NothingIdeas.db")
      const query = `DELETE FROM nothing_project_node WHERE node_id = ${nodeid}`
      const result = await db.execute(query);
      console.log(result)
      await db.close();
      this.fetchPreojectNodes();

    },
    // 清空表单数据
    clearForm() {
      this.form.name = ''
      this.editorContent = ''
      this.type_value = ''
      this.$refs.editor.setHTML('')
    },
    clearForm2() {
      this.form.name = ''
      this.editorContent = ''
      this.type_value = ''
      this.$refs.Editeditor.setHTML('')
    },
    // 获取节点数据
    async getNodeData(nodeType, nodeTitle, nodeText, nodeid) {
      this.node_item.node_type = nodeType;
      this.node_item.node_title = nodeTitle;
      this.node_item.node_text = nodeText;
      this.node_item.node_id = nodeid;
      console.log(this.node_item)
      this.dialogFormEditVisible = true;

      this.formEdit.name = nodeTitle;
      this.typeEdit_value = nodeType;
      console.log("测试testnode:"+nodeText)

      await this.$nextTick();

      this.$refs.Editeditor.setHTML(nodeText);

    },
    // 编辑按钮更新数据
    async updateNode() {
      const db = await Database.load("sqlite:NothingIdeas.db")

      const query = `UPDATE nothing_project_node SET node_title = ?, node_text = ?, node_type = ? WHERE node_id = ?`
      const params = [this.formEdit.name, this.$refs.Editeditor.getHTML(), this.typeEdit_value, this.node_item.node_id]
      const result = await db.execute(query, params);
      console.log(this.node_item.node_id)
      await db.close();
      this.fetchPreojectNodes();
      this.dialogFormEditVisible = false;
      // // 清空表单数据
      this.formEdit.name = ''
      this.editorContent2 = ''
      this.typeEdit_value = ''
      this.$refs.Editeditor.setHTML('')
      this.node_item = {};
    },
    // 将date转为”年-月-日“格式
    formatDate(date) {
      const year = date.getFullYear();
      const month = (date.getMonth() + 1).toString().padStart(2, '0');
      const day = date.getDate().toString().padStart(2, '0');
      return `${year}年${month}月${day}日`;
    }
  },
  setup() {

    // 新增按钮-对话框
    const dialogFormVisible = ref(false)
    const dialogFormEditVisible = ref(false)
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

    const formEdit = reactive({
      name: '',
      region: '',
      date1: '',
      date2: '',
      delivery: false,
      type: [],
      resource: '',
      desc: ''
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
      dialogFormEditVisible,
      dialogFormVisible,
      formLabelWidth,
      form,
      formEdit,
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
}
.el-main {
  overflow: unset !important;
}
input,
button {
  box-shadow: none !important;
}

h2 {
    font-size: 16px;
}

/* blockquote样式 */
blockquote {
    border-top-color: #42a4ff;
    border-top-width: 2px;
    border-top-style: dashed;
    border-bottom-style: dashed;
    border-bottom-color: #42a4ff;
    border-bottom-width: 2px;
    margin: 15px 12px;
    padding: 10px;
    color: #42a4ff;
}
pre.ql-syntax {
    width: 100%;
    white-space: pre-wrap;
    /* word-wrap: break-word; */
    /* overflow-wrap: break-word; */
    background-color: #42a4ff;
    color: aliceblue;
    font-size: 14px;
    border-radius: 10px;
    padding: 10px;
    margin: 10px 10px;
    width: 95%;
}
</style>
