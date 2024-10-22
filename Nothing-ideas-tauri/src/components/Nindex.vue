/** 首页 */
<template>
  <div class="common-layout">
    <el-container>
      <!-- <el-header>Header</el-header> -->


      <el-main>
        <el-card style="max-width: 70vw; margin: auto;">
          <!-- 页头 -->
          <template #header>
            <div class="card-header">
              <h1 class="header-title">NothingIdeas想法流程</h1>
              <div class="search-container">
                <el-input
                  v-model="searchKeyword"
                  placeholder="搜索项目..."
                  class="search-input"
                  @keyup.enter="performSearch"
                >
                  <template #append>
                    <el-button @click="performSearch" icon="el-icon-search">搜索</el-button>
                  </template>
                </el-input>
              </div>
              <el-button-group class="btn-group">
                <el-button type="primary" size="mini" @click="dialogFormVisible = true">新增</el-button>
                <el-button type="danger" size="mini" @click="showDeleteButton">删除</el-button>
              </el-button-group>
            </div>
          </template>

          <!-- 内容  -->
          <template v-for="item in displayedProjects" :key="item.project_id">
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
              <el-pagination 
                background 
                layout="prev, pager, next" 
                :total="filteredProjects.length" 
                :current-page.sync="currentPage"
                :page-size="pageSize" 
                @current-change="handleCurrentChange" 
              />

              <el-button-group class="btn-group">
                <el-button type="primary" size="mini" plain @click="allButton">全部</el-button>
                <el-button type="success" size="mini" plain @click="closedButton">已闭环</el-button>
                <el-button type="warning" size="mini" plain @click="unclosedButton">未闭环</el-button>
              </el-button-group>

            </div>
          </template>


          <!-- 新增-对话框 -->
          <el-dialog 
            v-model="dialogFormVisible" 
            title="新增NothingIdeas项目" 
            width="500"
          >
            <el-form :model="form" @submit.prevent="addProject">
              <el-form-item label="项目名称" :label-width="formLabelWidth">
                <el-input 
                  v-model="form.name" 
                  autocomplete="off" 
                  ref="newProjectInput"
                  @keyup.enter.prevent
                />
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
      allProjects: [],    // 存储所有项目的数组
      currentPage: 1,     // 当前页码
      pageSize: 10,       // 每页显示条数
      searchKeyword: '',  // 搜索关键词
      IsDeleted: false,      // 是否删除
      isAdding: false, // 新增：用于防止重复提交
    };
  },
  computed: {
    filteredProjects() {
      if (!this.searchKeyword) {
        return this.allProjects;
      }
      const keyword = this.searchKeyword.toLowerCase();
      return this.allProjects.filter(project => 
        project.project_title.toLowerCase().includes(keyword) ||
        project.project_status.toLowerCase().includes(keyword)
      );
    },
    displayedProjects() {
      const start = (this.currentPage - 1) * this.pageSize;
      const end = start + this.pageSize;
      return this.filteredProjects.slice(start, end);
    }
  },
  async created() {
    await this.fetchAllProjects();
    //监听数据加载完成事件，然后更新界面
    this.$onces('data-loaded', () => {
      this.$forceUpdate();// 强制刷新界面
    });
    
  },
  methods: {
    // -------------------------- 以下为分页相关方法 --------------------------
    async fetchAllProjects() {
      const db = await Database.load("sqlite:NothingIdeas.db");
      const query = `SELECT * FROM nothing_project ORDER BY project_create_time DESC`;
      this.allProjects = await db.select(query);
      await db.close();
    },
    handleCurrentChange(page) {
      this.currentPage = page;
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
      this.fetchAllProjects(); // 重新获取数据
    },
    // ---------- 以下为【已闭环按钮】的功能 ----------------
    async closedButton() {
      const db = await Database.load("sqlite:NothingIdeas.db");
      const query = `SELECT * FROM nothing_project where project_status = '已闭环'`;
      const NothingProject = await db.select(query);
      this.allProjects = NothingProject; // 更新数据列表
      await db.close();
    },
    // ---------- 以下为【未闭环按钮】的功能 ----------------
    async unclosedButton() {
      const db = await Database.load("sqlite:NothingIdeas.db");
      const query = `SELECT * FROM nothing_project where project_status = '未闭环'`;
      const NothingProject = await db.select(query);
      this.allProjects = NothingProject; // 更新数据列表
      await db.close();

    },
    // ---------- 以下为是否显示删除按钮的功能 ----------------
    showDeleteButton() {
      this.IsDeleted = !this.IsDeleted;// 切换删除按钮显示状态
    },
    //以下为是否确删除弹出窗口messageBox的功能
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
              this.fetchAllProjects(); // 重新获取数据
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
      if (this.isAdding) return; // 防止重复提交
      this.isAdding = true;

      if (!this.form.name.trim()) {
        // 如果项目名称为空,不执行添加操作
        this.isAdding = false;
        return;
      }

      try {
        const db = await Database.load("sqlite:NothingIdeas.db");
        const query = `INSERT INTO nothing_project (project_title, project_status, project_create_time) VALUES (?, ?, ?)`;
        const params = [this.form.name.trim(), '未闭环', new Date()];
        await db.execute(query, params);
        await db.close();
        this.dialogFormVisible = false; // 关闭对话框
        await this.fetchAllProjects(); // 重新获取数据
        this.form.name = ''; // 清空表单
        ElMessage.success('项目添加成功');
      } catch (error) {
        console.error('添加项目失败:', error);
        ElMessage.error('添加项目失败，请重试');
      } finally {
        this.isAdding = false;
      }
    },
    performSearch() {
      this.currentPage = 1; // 重置到第一页
      // 不需要重新获取数据,因为我们已经有了所有项目
    },

    watch: {
      dialogFormVisible(newVal) {
        if (newVal) {
          // 当对话框打开时,聚焦到输入框
          this.$nextTick(() => {
            this.$refs.newProjectInput.focus();
          });
        } else {
          // 当对话框关闭时，重置表单
          this.form.name = '';
          this.isAdding = false;
        }
      }
    }
  },
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

  },
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

.project-title[data-v-67cf175d] {
    top: 12px;
    font-size: 16px;
    font-weight: 500;
    color: #333;
    margin-left: 15px;
    cursor: pointer;
    transition: color 0.3s ease, transform 0.3s ease;
    display: inline-block;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
}

.header-title {
  margin: 10px 0;
  font-family: cursive;
  text-shadow: 0 0 black;
}

.search-container {
  flex-grow: 1;
  margin: 0 20px;
  max-width: 400px;
}

.search-input {
  width: 100%;
}
</style>


