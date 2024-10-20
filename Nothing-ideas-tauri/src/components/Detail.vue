<template>
  <div class="common-layout">
    <el-container>
      <!-- Header -->
      <el-header class="custom-header">
        <div class="header-left">
          <el-button class="back-button" :icon="Back" @click="$router.push('/')">返回</el-button>
          <h1 class="project-title">{{ this.projectname }}</h1>
        </div>

        <el-button-group class="header-right">
          <el-button type="primary" plain @click="dialogFormVisible = true">新增节点</el-button>
          <el-button type="success" plain :disabled="this.projectstaus === '已闭环'" @click="updateProjectStatus()">流程闭环</el-button>
          <el-button type="warning" plain :disabled="this.projectstaus === '未闭环'" @Click="recoverProjectStatus()">恢复流程</el-button>
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
                            <h3 class="node-title">{{ item.node_title }}</h3>
                            <div class="markdown-content" v-html="item.node_text"></div>
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
            @text-change="handleTextChange" class="markdown-style-editor"></quill-editor>

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
<!-- 回到顶部 -->
  <el-backtop :right="5" :bottom="100" />
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
      typeEdit_value: '',//编辑节点类型
      editorContent: '',
      options: [],
      project_id: null,
      NothingProjectNodes: [],
      NothingProject: [],
      node_item: { 'node_type': '', 'node_title': '', 'node_text': '', 'node_id': '' },
      projectname: '',
      projectstaus: '',
      editorOption: {
        placeholder: '请输入内容...',
        modules: {
          toolbar: [
            [{ 'header': [1, 2, 3, 4, 5, 6, false] }],
            ['bold', 'italic', 'underline', 'strike'],
            ['blockquote', 'code-block'],
            [{ 'list': 'ordered'}, { 'list': 'bullet' }],
            [{ 'script': 'sub'}, { 'script': 'super' }],
            [{ 'indent': '-1'}, { 'indent': '+1' }],
            [{ 'direction': 'rtl' }],
            [{ 'size': ['small', false, 'large', 'huge'] }],
            [{ 'color': [] }, { 'background': [] }],
            [{ 'font': [] }],
            [{ 'align': [] }],
            ['clean'],
            ['link', 'image']
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
      const query = `SELECT * FROM nothing_project_node WHERE project_id = ${this.project_id} order by node_create_time desc`
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
      // 清空表单数
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
    background-color: #a0cfff;
    color: white;
    font-size: 14px;
    border-radius: 10px;
    padding: 10px;
    margin: 10px 10px;
    width: 95%;
}
img {
    max-width: 100%;
    border-radius: 5px;
}

.custom-header {
  background-color: #ffffff;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 20px;
  height: 60px;
}

.header-left {
  display: flex;
  align-items: center;
}

.back-button {
  margin-right: 20px;
}

.project-title {
  font-size: 1.2em;
  font-weight: 500;
  color: #333333;
  margin: 0;
  padding: 0 10px;
  position: relative;
  transition: color 0.3s ease;
}

.project-title::after {
  content: '';
  position: absolute;
  bottom: -3px;
  left: 10px;
  width: calc(100% - 20px);
  height: 2px;
  background-color: #409EFF;
  transform: scaleX(0);
  transition: transform 0.3s ease;
}

.project-title:hover {
  color: #409EFF;
}

.project-title:hover::after {
  transform: scaleX(1);
}

.header-right .el-button {
  margin-left: 10px;
}

.el-button {
  border-radius: 4px;
}

/* 可以根据需要调整按钮的具体样式 */
.el-button--primary.is-plain {
  color: #409EFF;
  border-color: #b3d8ff;
  background-color: #ecf5ff;
}

.el-button--success.is-plain {
  color: #67C23A;
  border-color: #c2e7b0;
  background-color: #f0f9eb;
}

.el-button--warning.is-plain {
  color: #E6A23C;
  border-color: #f5dab1;
  background-color: #fdf6ec;
}

/* 优化 Markdown 样式 */
.markdown-content {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen', 'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue', sans-serif;
  line-height: 1.8;
  color: #333;
  word-wrap: break-word;
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
}

.markdown-content h1,
.markdown-content h2,
.markdown-content h3,
.markdown-content h4,
.markdown-content h5,
.markdown-content h6 {
  margin-top: 1.5em;
  margin-bottom: 0.75em;
  font-weight: 600;
  line-height: 1.25;
  color: #2c3e50;
}

.markdown-content h1 { font-size: 2em; border-bottom: 1px solid #eaecef; padding-bottom: 0.3em; }
.markdown-content h2 { font-size: 1.75em; border-bottom: 1px solid #eaecef; padding-bottom: 0.3em; }
.markdown-content h3 { font-size: 1.5em; }
.markdown-content h4 { font-size: 1.25em; }
.markdown-content h5 { font-size: 1.1em; }
.markdown-content h6 { font-size: 1em; color: #6a737d; }

.markdown-content p {
  font-family: -apple-system, "PingFang SC", "Hiragino Sans GB", "Microsoft YaHei", "Helvetica Neue", Arial, sans-serif;
  font-size: 16px;
  margin-top: 0;
  margin-bottom: 1.2em;
  line-height: 1.6;
}

.markdown-content a {
  color: #0366d6;
  text-decoration: none;
  border-bottom: 1px solid #0366d6;
  transition: border-bottom 0.3s ease;
}

.markdown-content a:hover {
  border-bottom: 2px solid #0366d6;
}

.markdown-content strong {
  font-weight: 600;
}

.markdown-content img {
  max-width: 100%;
  box-sizing: border-box;
  background-color: #fff;
  border-radius: 4px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.markdown-content code {
  padding: 0.2em 0.4em;
  margin: 0;
  font-size: 85%;
  background-color: rgba(27,31,35,0.05);
  border-radius: 3px;
  font-family: SFMono-Regular, Consolas, 'Liberation Mono', Menlo, monospace;
}

.markdown-content pre {
  padding: 16px;
  overflow: auto;
  font-size: 85%;
  line-height: 1.45;
  background-color: #f6f8fa;
  border-radius: 6px;
  box-shadow: inset 0 1px 0 rgba(255,255,255,0.6);
}

.markdown-content pre code {
  display: inline;
  max-width: auto;
  padding: 0;
  margin: 0;
  overflow: visible;
  line-height: inherit;
  word-wrap: normal;
  background-color: transparent;
  border: 0;
}

.markdown-content blockquote {
  padding: 0 1em;
  color: #6a737d;
  border-left: 0.25em solid #dfe2e5;
  margin: 0 0 16px 0;
  font-style: italic;
}

.markdown-content ul,
.markdown-content ol {
  padding-left: 2em;
  margin-top: 0;
  margin-bottom: 16px;
}

.markdown-content li {
  font-size: 16px;  /* 从 15px 改为 16px */
  margin-bottom: 0.5em;
}

.markdown-content table {
  display: block;
  width: 100%;
  overflow: auto;
  border-spacing: 0;
  border-collapse: collapse;
  margin-bottom: 16px;
}

.markdown-content table th,
.markdown-content table td {
  padding: 8px 13px;
  border: 1px solid #dfe2e5;
}

.markdown-content table th {
  font-weight: 600;
  background-color: #f6f8fa;
}

.markdown-content table tr {
  background-color: #fff;
  border-top: 1px solid #c6cbd1;
}

.markdown-content table tr:nth-child(2n) {
  background-color: #f8f8f8;
}

/* Quill 编辑器样式 */
.markdown-style-editor {
  font-family: -apple-system, "PingFang SC", "Hiragino Sans GB", "Microsoft YaHei", "Helvetica Neue", Arial, sans-serif;
  line-height: 1.8;
  color: #333;
}

.ql-editor {
  font-size: 16px;
  line-height: 1.6;
  color: #34495e;
}

.ql-editor p {
  font-size: 16px;
  margin-bottom: 1.2em;
  line-height: 1.6;
}

.ql-editor ol,
.ql-editor ul {
  padding-left: 2em;
  margin-top: 0;
  margin-bottom: 16px;
}

.ql-editor li {
  font-size: 16px;
  margin-bottom: 0.5em;
}

.ql-editor pre {
  background-color: #f8f8f8;
  padding: 15px;
  border-radius: 5px;
  overflow-x: auto;
  font-size: 14px;
  line-height: 1.45;
}

.ql-editor blockquote {
  border-left: 4px solid #3498db;
  padding-left: 15px;
  color: #7f8c8d;
  font-style: italic;
  margin: 0 0 16px 0;
}

.ql-editor h1,
.ql-editor h2,
.ql-editor h3,
.ql-editor h4,
.ql-editor h5,
.ql-editor h6 {
  margin-top: 1.5em;
  margin-bottom: 0.75em;
  font-weight: 600;
  line-height: 1.25;
  color: #2c3e50;
}

.ql-editor h1 { font-size: 2em; border-bottom: 1px solid #eaecef; padding-bottom: 0.3em; }
.ql-editor h2 { font-size: 1.75em; border-bottom: 1px solid #eaecef; padding-bottom: 0.3em; }
.ql-editor h3 { font-size: 1.5em; }
.ql-editor h4 { font-size: 1.25em; }
.ql-editor h5 { font-size: 1.1em; }
.ql-editor h6 { font-size: 1em; color: #6a737d; }

.ql-editor a {
  color: #3498db;
  text-decoration: none;
  border-bottom: 1px solid transparent;
  transition: border-color 0.3s ease;
}

.ql-editor a:hover {
  border-bottom-color: #3498db;
}

.ql-editor code {
  background-color: #f8f8f8;
  padding: 2px 4px;
  border-radius: 3px;
  font-family: Consolas, Monaco, 'Andale Mono', 'Ubuntu Mono', monospace;
  font-size: 0.9em;
}

.ql-editor img {
  max-width: 100%;
  box-sizing: border-box;
  background-color: #fff;
  border-radius: 4px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

/* 其他样式保持不变 */

.node-title {
  font-size: 1.4em;
  font-weight: 600;
  color: #2c3e50;
  margin-top: 0;
  margin-bottom: 15px;
  padding-bottom: 10px;
  border-bottom: 2px solid #ecf0f1;
  transition: color 0.3s ease;
}

.node-title:hover {
  color: #3498db;
}

.custom-card {
  margin-bottom: 20px;
  border-radius: 8px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  transition: box-shadow 0.3s ease;
}

.custom-card:hover {
  box-shadow: 0 6px 8px rgba(0, 0, 0, 0.15);
}

.markdown-content {
  font-family: -apple-system, "PingFang SC", "Hiragino Sans GB", "Microsoft YaHei", "Helvetica Neue", Arial, sans-serif;
  font-size: 15px;
  line-height: 1.6;
  color: #34495e;
}

.markdown-content h1 { font-size: 1.8em; }
.markdown-content h2 { font-size: 1.5em; }
.markdown-content h3 { font-size: 1.3em; }
.markdown-content h4 { font-size: 1.2em; }
.markdown-content h5 { font-size: 1.1em; }
.markdown-content h6 { font-size: 1em; }

.markdown-content p {
  margin-bottom: 1em;
}

.markdown-content a {
  color: #3498db;
  text-decoration: none;
  border-bottom: 1px solid transparent;
  transition: border-color 0.3s ease;
}

.markdown-content a:hover {
  border-bottom-color: #3498db;
}

.markdown-content code {
  background-color: #f8f8f8;
  padding: 2px 4px;
  border-radius: 3px;
  font-family: Consolas, Monaco, 'Andale Mono', 'Ubuntu Mono', monospace;
  font-size: 0.9em;
}

.markdown-content pre {
  background-color: #f8f8f8;
  padding: 15px;
  border-radius: 5px;
  overflow-x: auto;
}

.markdown-content blockquote {
  border-left: 4px solid #3498db;
  padding-left: 15px;
  color: #7f8c8d;
  font-style: italic;
}

/* ... 其他样式保持不变 ... */
</style>



