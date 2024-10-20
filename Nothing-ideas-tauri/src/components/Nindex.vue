/** 首页 */
<template>
  <div class="common-layout">
    <el-container>
      <!-- <el-header>Header</el-header> -->


      <el-main>
        <el-card style="max-width: 70vw; margin: auto;">
          <!-- 页头 -->
          <template #header>
            <div class="card-header" style="display: flex; justify-content: space-between; align-items: center;">
              <h1 class="header-title" style="margin: 10px;font-family: cursive; text-shadow: 0 0 black;">NothingIdeas想法流程
              </h1>
              <el-button-group class="btn-group">
                <el-button type="primary" size="mini" @click="dialogFormVisible = true">新增</el-button>
                <el-button type="danger" size="mini" @click="showDeleteButton">删除</el-button>
                <!-- <el-button size="mini" @click="testButton">测试按钮</el-button> -->
              </el-button-group>
            </div>
          </template>

          <!-- 内容  -->
          <template v-for="item in NothingProject">
            <div
              style="display: flex; justify-content: space-between; align-items: center; margin-top: 15px; margin-bottom: 15px;">
              <div class="item" style=" align-items: center;">
                <el-button plain size="mini" style="width: 60px;">{{ daysSinceCreation(new
                  Date(item.project_create_time)) }}天</el-button>
                <el-button plain :type="item.project_status === '未闭环' ? 'warning' : 'success'" size="mini">{{
                  item.project_status }}</el-button>
                <span class="project-title" @click="$router.push('/Detail/' + item.project_id)">
                  {{ item.project_title }}
                </span>
              </div>
              <el-button type="danger" link v-if="IsDeleted" @click="deleteProjectconfirm(item.project_id)">X</el-button>
            </div>
          </template>

          <!-- 页脚 -->
          <template #footer>

            <div style="display: flex; justify-content: space-between; align-items: center;">
              <el-pagination background layout="prev, pager, next" :total="totalItems" :current-page.sync="currentPage"
                :page-size="pageSize" @current-change="handleCurrentChange" />

              <el-button-group class="btn-group">
                <el-button type="primary" size="mini" plain @click="allButton">全部</el-button>
                <el-button type="success" size="mini" plain @click="closedButton">已闭环</el-button>
                <el-button type="warning" size="mini" plain @click="unclosedButton">未闭环</el-button>
              </el-button-group>

            </div>
          </template>


          <!-- 新增-对话框 -->
          <el-dialog v-model="dialogFormVisible" title="新增NothingIdeas项目" width="500">
            <el-form :model="form">
              <el-form-item label="项目名称" :label-width="formLabelWidth">
                <el-input v-model="form.name" autocomplete="off" />
              </el-form-item>
            </el-form>
            <template #footer>
              <div class="dialog-footer">
                <el-button @click="dialogFormVisible = false">取消</el-button>
                <el-button type="primary" @click="addProject">
                  确定
                </el-button>
              </div>
            </template>
          </el-dialog>
        </el-card>
      </el-main>
    </el-container>
  </div>
</template>
  

<script>
import Database from "tauri-plugin-sql-api";
import { ElMessage, ElMessageBox } from 'element-plus';
import { reactive, ref } from 'vue'


export default {
  data() {
    return {
      NothingProject: [],    // 数据列表
      totalItems: 0,         // 总数据条数
      currentPage: 1,        // 当前页码
      pageSize: 10,           // 每页显示条数
      IsDeleted: false,      // 是否删除
    };
  },
  computed: {
    // 根据当前页码和每页显示条数，计算出当前页的数据
    paginatedProjects() {
      const startIndex = (this.currentPage - 1) * this.pageSize;
      const endIndex = startIndex + this.pageSize;
      return this.NothingProject.slice(startIndex, endIndex);
    }

  },
  async created() {
    await this.fetchProjects();
    //监听数据加载完成事件，然后更新界面
    this.$onces('data-loaded', () => {
      this.$forceUpdate();// 强制刷新界面
    });
    
  },
  methods: {
    // -------------------------- 以下为分页相关方法 --------------------------
    async fetchProjects() {
      const db = await Database.load("sqlite:NothingIdeas.db");

      // 查询总数据条数
      const totalCount = await db.select("SELECT COUNT(*) as count FROM nothing_project");
      this.totalItems = totalCount[0].count;

      // 查询当前页数据
      const offset = (this.currentPage - 1) * this.pageSize;
      const query = `SELECT * FROM nothing_project order by project_create_time desc LIMIT ${this.pageSize} OFFSET ${offset}`;
      const NothingProject = await db.select(query);

      this.NothingProject = NothingProject; // 更新数据列表
      await db.close();
      // 数据加载完成后触发刷新事件
      this.$emit('data-loaded');
    },
    async handleCurrentChange(page) {
      this.currentPage = page; // 更新当前页码
      await this.fetchProjects(); // 重新获取数据
    },
    // -------------------------- 以下为测试按钮相关方法 ----------------
    async testButton() {
      const db = await Database.load("sqlite:NothingIdeas.db");
      // 执行测试按钮操作，例如插入数据等
      console.log("测试按钮被点击");

      const time = new Date(2024, 3, 9); // 假设插入时间为 2024-04-09
      // 插入一条数据
      await db.execute("INSERT INTO test (name, age,time) VALUES (?,?,?)", ["Alice", 25, time]);
      await db.close();
      console.log("数据插入成功");
    },
    // ---------- 以下为路由相关方法 ----------------
    goToDetail(item) {
      this.$router.push(`/Detail?id=${item.id}`); // 跳转到详情页面，传递项目 ID
    },
    // ---------- 以下为时间计算方法daysSinceCreation(date) ----------------
    daysSinceCreation(date) {
      const days = Math.floor((new Date() - date) / (1000 * 60 * 60 * 24));
      return days;
    },
    // ---------- 以下为【全部按钮】的功能 ----------------
    allButton() {
      this.currentPage = 1; // 回到第一页
      this.fetchProjects(); // 重新获取数据
    },
    // ---------- 以下为【已闭环按钮】的功能 ----------------
    async closedButton() {
      const db = await Database.load("sqlite:NothingIdeas.db");
      const query = `SELECT * FROM nothing_project where project_status = '已闭环'`;
      const NothingProject = await db.select(query);
      this.NothingProject = NothingProject; // 更新数据列表
      await db.close();
    },
    // ---------- 以下为【未闭环按钮】的功能 ----------------
    async unclosedButton() {
      const db = await Database.load("sqlite:NothingIdeas.db");
      const query = `SELECT * FROM nothing_project where project_status = '未闭环'`;
      const NothingProject = await db.select(query);
      this.NothingProject = NothingProject; // 更新数据列表
      await db.close();

    },
    // ---------- 以下为是否显示删除按钮的功能 ----------------
    showDeleteButton() {
      this.IsDeleted = !this.IsDeleted;// 切换删除按钮显示状态
    },
    //以下为是否确认删除弹出窗口messageBox的功能
    async deleteProjectconfirm(projectId) {
      ElMessageBox.confirm(
        '是否确定删除这条数据?',
        '删除操作',
        {
          confirmButtonText: '确定',
          cancelButtonText: '取消',
          type: 'warning',
          beforeClose: async (action, instance, done) => {
            if (action === 'confirm') {
              const db = await Database.load("sqlite:NothingIdeas.db");
              const query = `DELETE FROM nothing_project WHERE project_id = ${projectId}`;
              await db.execute(query);
              await db.close();
              this.fetchProjects(); // 重新获取数据
              done();

            } else if (action === 'cancel') {
              done();
            }
          }

        }
      )
        .then(() => {
          ElMessage({
            type: 'success',
            message: '删除成功',
          })
        })
        .catch(() => {
          ElMessage({
            type: 'info',
            message: '已取消',
          })
        })
    },
    // ---------- 以下为新增项目的功能 ----------------
    async addProject() {
      const db = await Database.load("sqlite:NothingIdeas.db");
      const query = `INSERT INTO nothing_project (project_title, project_status, project_create_time) VALUES (?, ?, ?)`;
      const params = [this.form.name, '未闭环', new Date()];
      await db.execute(query, params);
      await db.close();
      this.dialogFormVisible = false; // 关闭对话框
      this.fetchProjects(); // 重新获取数据
  }},
  setup() {
    // ---------- 以下为新增项目的功能 ----------------
    const dialogFormVisible = ref(false)
    const formLabelWidth = '140px'

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

    return {
      dialogFormVisible,
      formLabelWidth,
      form
    };

  }
}
</script>

<style scoped>
.project-title {
  font-size: 16px;
  font-weight: 500;
  color: #333;
  margin-left: 15px;
  cursor: pointer;
  transition: color 0.3s ease, transform 0.3s ease;
  display: inline-block;
}

.project-title:hover {
  color: #409EFF;
  animation: bounce 0.5s ease;
}

@keyframes bounce {
  0%, 100% {
    transform: translateX(0);
  }
  25% {
    transform: translateX(-5px);
  }
  75% {
    transform: translateX(5px);
  }
}
</style>
