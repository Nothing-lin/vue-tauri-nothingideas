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
          <el-button size="mini" plain style="box-shadow: none;">测试按钮{{ project_id }}</el-button>
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

                  <el-timeline-item timestamp="2018年4月12日" placement="top">
                    <template v-slot:dot>
                      <!-- 使用 dot 插槽自定义小圆点 -->
                      <span class="custom-dot">主题</span>
                    </template>
                    <el-card class="custom-card">
                      <el-row :gutter="20">
                        <el-col :span="16">
                          <div class="grid-content bg-purple">
                            <h4>学习如何搭建第一个vue开发框架</h4>
                            <p>王小虎 提交于 2018/4/12 20:46</p>
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

                  <el-timeline-item timestamp="2018年4月12日" placement="top">
                    <template v-slot:dot>
                      <!-- 使用 dot 插槽自定义小圆点 -->
                      <span class="custom-dot" style="background-color: rgb(255, 107, 53);">拓展</span>
                    </template>
                    <el-card class="custom-card">
                      <el-row :gutter="20">
                        <el-col :span="16">
                          <div class="grid-content bg-purple">
                            <h4>学习如何搭建第一个vue开发框架</h4>
                            <p>王小虎 提交于 2018/4/12 20:46</p>
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

                  <el-timeline-item timestamp="2018年4月12日" placement="top">
                    <template v-slot:dot>
                      <!-- 使用 dot 插槽自定义小圆点 -->
                      <span class="custom-dot">主题</span>
                    </template>
                    <el-card class="custom-card">
                      <el-row :gutter="20">
                        <el-col :span="16">
                          <div class="grid-content bg-purple">
                            <h4>学习如何搭建第一个vue开发框架</h4>
                            <p>王小虎 提交于 2018/4/12 20:46</p>
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
      <el-dialog v-model="dialogFormVisible" title="新增节点" width="500">
        <el-form :model="form">
          <el-form-item label="节点名称" :label-width="formLabelWidth">
            <el-input v-model="form.name" autocomplete="off" />
          </el-form-item>
          <el-form-item label="Zones" :label-width="formLabelWidth">
            <el-select v-model="form.region" placeholder="Please select a zone">
              <el-option label="Zone No.1" value="shanghai" />
              <el-option label="Zone No.2" value="beijing" />
            </el-select>
          </el-form-item>
        </el-form>
        <template #footer>
          <div class="dialog-footer">
            <el-button @click="dialogFormVisible = false">Cancel</el-button>
            <el-button type="primary" @click="dialogFormVisible = false">
              Confirm
            </el-button>
          </div>
        </template>
      </el-dialog>
    </el-container>
  </div>
</template>


<script>
import { Back, Edit, Search, Share, Upload } from '@element-plus/icons-vue'
import { reactive, ref } from 'vue'

export default {
  data() {
    return {
      project_id:null
    }
  },
    mounted() {
      this.project_id = this.$route.params.project_id//获取路由参数
      console.log(this.project_id)
    },
  setup() {

  // 新增按钮-对话框
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
