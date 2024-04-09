/** 首页 */
<template>
    <div class="common-layout">
        <el-container>
            <el-header>Header</el-header>


            <el-main>
                <el-card style="max-width: 70vw; margin: auto;">
                    <!-- 页头 -->
                    <template #header>
                        <div class="card-header" style="display: flex; justify-content: space-between; align-items: center;">
                            <span class="header-title">nothingIdeas想法流程</span>
                            <el-button-group class="btn-group">
                                <el-button type="primary" size="mini">新增</el-button>
                                <el-button type="danger" size="mini">删除</el-button>
                            </el-button-group>
                        </div>
                    </template>

                    <!-- 内容  -->
                    <template v-for="n in 10">
                        <div class="item" style="margin-top: 15px; margin-bottom: 15px;">
                            <el-button plain size="mini">Button 1</el-button>
                            <el-button plain :type="n%2 === 0? 'danger' : 'success'" size="mini">Button 2</el-button>
                            <span class="text" style="margin-left: 10px;">Text</span>
                        </div>
                    </template>

                    <!-- 页脚 -->
                    <template #footer>
                        
                        <div style="display: flex; justify-content: space-between; align-items: center;">
                            <el-pagination background layout="prev, pager, next" :total="1000" />

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
  