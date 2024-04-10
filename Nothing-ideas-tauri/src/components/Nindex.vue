/** 首页 */
<template>
    <div class="common-layout">
        <el-container>
            <el-header >Header</el-header>


            <el-main>
                <el-card style="max-width: 70vw; margin: auto;">
                    <!-- 页头 -->
                    <template #header>
                        <div class="card-header"
                            style="display: flex; justify-content: space-between; align-items: center;">
                            <span class="header-title">nothingIdeas想法流程</span>
                            <el-button-group class="btn-group">
                                <el-button type="primary" size="mini">新增</el-button>
                                <el-button type="danger" size="mini">删除</el-button>
                                <el-button size="mini" @click="MyDB">测试按钮</el-button>
                            </el-button-group>
                        </div>
                    </template>

                    <!-- 内容  -->
                    <template v-for="item in NothingProject">
                        <div class="item" style="margin-top: 15px; margin-bottom: 15px;">
                            <el-button plain size="mini">Button 1</el-button>
                            <el-button plain :type="item.project_status === '未闭环' ? 'danger' : 'success'" size="mini">{{item.project_status}}</el-button>
                            <span class="text" style="margin-left: 10px; cursor: pointer;"
                                @click="$router.push('/Detail')">{{item.project_title}}</span>
                        </div>
                    </template>

                    <!-- 页脚 -->
                    <template #footer>

                        <div style="display: flex; justify-content: space-between; align-items: center;">
                            <el-pagination 
                                background 
                                layout="prev, pager, next" 
                                :total="totalItems"
                                :current-page.sync="currentPage"
                                :page-size="pageSize"
                                @current-change="handleCurrentChange"
                                 />

                            <el-button-group class="btn-group">
                                <el-button type="primary" size="mini" plain>全部</el-button>
                                <el-button type="success" size="mini" plain>已闭环</el-button>
                                <el-button type="warning" size="mini" plain>未闭环</el-button>
                            </el-button-group>

                        </div>
                    </template>
                </el-card>
            </el-main>
        </el-container>
    </div>
</template>
  

<script>
import Database from "tauri-plugin-sql-api";

export default {
  data() {
    return {
      NothingProject: [],    // 数据列表
      totalItems: 0,         // 总数据条数
      currentPage: 1,        // 当前页码
      pageSize: 10           // 每页显示条数
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
  },
  methods: {
    async fetchProjects() {
      const db = await Database.load("sqlite:NothingIdeas.db");

      // 查询总数据条数
      const totalCount = await db.select("SELECT COUNT(*) as count FROM nothing_project");
      this.totalItems = totalCount[0].count;

      // 查询当前页数据
      const offset = (this.currentPage - 1) * this.pageSize;
      const query = `SELECT * FROM nothing_project LIMIT ${this.pageSize} OFFSET ${offset}`;
      const NothingProject = await db.select(query);

      this.NothingProject = NothingProject; // 更新数据列表
      await db.close();
    },
    async handleCurrentChange(page) {
      this.currentPage = page; // 更新当前页码
      await this.fetchProjects(); // 重新获取数据
    },
    async testButton() {
      const db = await Database.load("sqlite:NothingIdeas.db");
      // 执行测试按钮操作，例如插入数据等
      await db.close();
    },
    goToDetail(item) {
      this.$router.push(`/Detail?id=${item.id}`); // 跳转到详情页面，传递项目 ID
    }
  }
};
</script>