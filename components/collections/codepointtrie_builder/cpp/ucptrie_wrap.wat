(module
  (type (;0;) (func))
  (type (;1;) (func (param i32) (result i32)))
  (type (;2;) (func (param i32)))
  (type (;3;) (func (param i32 i32) (result i32)))
  (type (;4;) (func (param i32 i32 i32) (result i32)))
  (type (;5;) (func (result i32)))
  (type (;6;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;7;) (func (param i32 i32 i32 i32)))
  (type (;8;) (func (param i32 i32)))
  (type (;9;) (func (param i32 i32 i32 i32 i32 i32 i32) (result i32)))
  (type (;10;) (func (param i32 i32 i32 i32 i32) (result i32)))
  (type (;11;) (func (param i32 i32 i32 i32 i32)))
  (type (;12;) (func (param i32 i32 i32 i32 i32 i32) (result i32)))
  (func (;0;) (type 0))
  (func (;1;) (type 0)
    call 0)
  (func (;2;) (type 1) (param i32) (result i32)
    local.get 0
    call 3)
  (func (;3;) (type 1) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    block  ;; label = @1
      i32.const 0
      i32.load offset=1072
      local.tee 2
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          i32.const 0
          i32.load offset=1520
          local.tee 3
          i32.eqz
          br_if 0 (;@3;)
          i32.const 0
          i32.load offset=1524
          local.set 4
          br 1 (;@2;)
        end
        i32.const 0
        i64.const -1
        i64.store offset=1532 align=4
        i32.const 0
        i64.const 281474976776192
        i64.store offset=1524 align=4
        i32.const 0
        local.get 1
        i32.const 8
        i32.add
        i32.const -16
        i32.and
        i32.const 1431655768
        i32.xor
        local.tee 3
        i32.store offset=1520
        i32.const 0
        i32.const 0
        i32.store offset=1540
        i32.const 0
        i32.const 0
        i32.store offset=1492
        i32.const 65536
        local.set 4
      end
      i32.const 0
      local.set 2
      i32.const 131072
      i32.const 67088
      local.get 4
      i32.add
      i32.const -1
      i32.add
      i32.const 0
      local.get 4
      i32.sub
      i32.and
      i32.const 131072
      select
      i32.const 67088
      i32.sub
      local.tee 5
      i32.const 89
      i32.lt_u
      br_if 0 (;@1;)
      i32.const 0
      local.set 4
      i32.const 0
      local.get 5
      i32.store offset=1500
      i32.const 0
      i32.const 67088
      i32.store offset=1496
      i32.const 0
      i32.const 67088
      i32.store offset=1064
      i32.const 0
      local.get 3
      i32.store offset=1084
      i32.const 0
      i32.const -1
      i32.store offset=1080
      loop  ;; label = @2
        local.get 4
        i32.const 1108
        i32.add
        local.get 4
        i32.const 1096
        i32.add
        local.tee 3
        i32.store
        local.get 3
        local.get 4
        i32.const 1088
        i32.add
        local.tee 6
        i32.store
        local.get 4
        i32.const 1100
        i32.add
        local.get 6
        i32.store
        local.get 4
        i32.const 1116
        i32.add
        local.get 4
        i32.const 1104
        i32.add
        local.tee 6
        i32.store
        local.get 6
        local.get 3
        i32.store
        local.get 4
        i32.const 1124
        i32.add
        local.get 4
        i32.const 1112
        i32.add
        local.tee 3
        i32.store
        local.get 3
        local.get 6
        i32.store
        local.get 4
        i32.const 1120
        i32.add
        local.get 3
        i32.store
        local.get 4
        i32.const 32
        i32.add
        local.tee 4
        i32.const 256
        i32.ne
        br_if 0 (;@2;)
      end
      i32.const 67088
      i32.const -8
      i32.const 67088
      i32.sub
      i32.const 15
      i32.and
      i32.const 0
      i32.const 67088
      i32.const 8
      i32.add
      i32.const 15
      i32.and
      select
      local.tee 4
      i32.add
      local.tee 2
      i32.const 4
      i32.add
      local.get 5
      i32.const -56
      i32.add
      local.tee 3
      local.get 4
      i32.sub
      local.tee 4
      i32.const 1
      i32.or
      i32.store
      i32.const 0
      i32.const 0
      i32.load offset=1536
      i32.store offset=1076
      i32.const 0
      local.get 4
      i32.store offset=1060
      i32.const 0
      local.get 2
      i32.store offset=1072
      i32.const 67088
      local.get 3
      i32.add
      i32.const 56
      i32.store offset=4
    end
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 0
                            i32.const 236
                            i32.gt_u
                            br_if 0 (;@12;)
                            block  ;; label = @13
                              i32.const 0
                              i32.load offset=1048
                              local.tee 7
                              i32.const 16
                              local.get 0
                              i32.const 19
                              i32.add
                              i32.const -16
                              i32.and
                              local.get 0
                              i32.const 11
                              i32.lt_u
                              select
                              local.tee 5
                              i32.const 3
                              i32.shr_u
                              local.tee 3
                              i32.shr_u
                              local.tee 4
                              i32.const 3
                              i32.and
                              i32.eqz
                              br_if 0 (;@13;)
                              block  ;; label = @14
                                block  ;; label = @15
                                  local.get 4
                                  i32.const 1
                                  i32.and
                                  local.get 3
                                  i32.or
                                  i32.const 1
                                  i32.xor
                                  local.tee 6
                                  i32.const 3
                                  i32.shl
                                  local.tee 3
                                  i32.const 1088
                                  i32.add
                                  local.tee 4
                                  local.get 3
                                  i32.const 1096
                                  i32.add
                                  i32.load
                                  local.tee 3
                                  i32.load offset=8
                                  local.tee 5
                                  i32.ne
                                  br_if 0 (;@15;)
                                  i32.const 0
                                  local.get 7
                                  i32.const -2
                                  local.get 6
                                  i32.rotl
                                  i32.and
                                  i32.store offset=1048
                                  br 1 (;@14;)
                                end
                                local.get 4
                                local.get 5
                                i32.store offset=8
                                local.get 5
                                local.get 4
                                i32.store offset=12
                              end
                              local.get 3
                              i32.const 8
                              i32.add
                              local.set 4
                              local.get 3
                              local.get 6
                              i32.const 3
                              i32.shl
                              local.tee 6
                              i32.const 3
                              i32.or
                              i32.store offset=4
                              local.get 3
                              local.get 6
                              i32.add
                              local.tee 3
                              local.get 3
                              i32.load offset=4
                              i32.const 1
                              i32.or
                              i32.store offset=4
                              br 12 (;@1;)
                            end
                            local.get 5
                            i32.const 0
                            i32.load offset=1056
                            local.tee 8
                            i32.le_u
                            br_if 1 (;@11;)
                            block  ;; label = @13
                              local.get 4
                              i32.eqz
                              br_if 0 (;@13;)
                              block  ;; label = @14
                                block  ;; label = @15
                                  local.get 4
                                  local.get 3
                                  i32.shl
                                  i32.const 2
                                  local.get 3
                                  i32.shl
                                  local.tee 4
                                  i32.const 0
                                  local.get 4
                                  i32.sub
                                  i32.or
                                  i32.and
                                  local.tee 4
                                  i32.const -1
                                  i32.add
                                  local.get 4
                                  i32.const -1
                                  i32.xor
                                  i32.and
                                  local.tee 4
                                  local.get 4
                                  i32.const 12
                                  i32.shr_u
                                  i32.const 16
                                  i32.and
                                  local.tee 4
                                  i32.shr_u
                                  local.tee 3
                                  i32.const 5
                                  i32.shr_u
                                  i32.const 8
                                  i32.and
                                  local.tee 6
                                  local.get 4
                                  i32.or
                                  local.get 3
                                  local.get 6
                                  i32.shr_u
                                  local.tee 4
                                  i32.const 2
                                  i32.shr_u
                                  i32.const 4
                                  i32.and
                                  local.tee 3
                                  i32.or
                                  local.get 4
                                  local.get 3
                                  i32.shr_u
                                  local.tee 4
                                  i32.const 1
                                  i32.shr_u
                                  i32.const 2
                                  i32.and
                                  local.tee 3
                                  i32.or
                                  local.get 4
                                  local.get 3
                                  i32.shr_u
                                  local.tee 4
                                  i32.const 1
                                  i32.shr_u
                                  i32.const 1
                                  i32.and
                                  local.tee 3
                                  i32.or
                                  local.get 4
                                  local.get 3
                                  i32.shr_u
                                  i32.add
                                  local.tee 3
                                  i32.const 3
                                  i32.shl
                                  local.tee 4
                                  i32.const 1088
                                  i32.add
                                  local.tee 6
                                  local.get 4
                                  i32.const 1096
                                  i32.add
                                  i32.load
                                  local.tee 4
                                  i32.load offset=8
                                  local.tee 0
                                  i32.ne
                                  br_if 0 (;@15;)
                                  i32.const 0
                                  local.get 7
                                  i32.const -2
                                  local.get 3
                                  i32.rotl
                                  i32.and
                                  local.tee 7
                                  i32.store offset=1048
                                  br 1 (;@14;)
                                end
                                local.get 6
                                local.get 0
                                i32.store offset=8
                                local.get 0
                                local.get 6
                                i32.store offset=12
                              end
                              local.get 4
                              local.get 5
                              i32.const 3
                              i32.or
                              i32.store offset=4
                              local.get 4
                              local.get 3
                              i32.const 3
                              i32.shl
                              local.tee 3
                              i32.add
                              local.get 3
                              local.get 5
                              i32.sub
                              local.tee 6
                              i32.store
                              local.get 4
                              local.get 5
                              i32.add
                              local.tee 0
                              local.get 6
                              i32.const 1
                              i32.or
                              i32.store offset=4
                              block  ;; label = @14
                                local.get 8
                                i32.eqz
                                br_if 0 (;@14;)
                                local.get 8
                                i32.const -8
                                i32.and
                                i32.const 1088
                                i32.add
                                local.set 5
                                i32.const 0
                                i32.load offset=1068
                                local.set 3
                                block  ;; label = @15
                                  block  ;; label = @16
                                    local.get 7
                                    i32.const 1
                                    local.get 8
                                    i32.const 3
                                    i32.shr_u
                                    i32.shl
                                    local.tee 9
                                    i32.and
                                    br_if 0 (;@16;)
                                    i32.const 0
                                    local.get 7
                                    local.get 9
                                    i32.or
                                    i32.store offset=1048
                                    local.get 5
                                    local.set 9
                                    br 1 (;@15;)
                                  end
                                  local.get 5
                                  i32.load offset=8
                                  local.set 9
                                end
                                local.get 9
                                local.get 3
                                i32.store offset=12
                                local.get 5
                                local.get 3
                                i32.store offset=8
                                local.get 3
                                local.get 5
                                i32.store offset=12
                                local.get 3
                                local.get 9
                                i32.store offset=8
                              end
                              local.get 4
                              i32.const 8
                              i32.add
                              local.set 4
                              i32.const 0
                              local.get 0
                              i32.store offset=1068
                              i32.const 0
                              local.get 6
                              i32.store offset=1056
                              br 12 (;@1;)
                            end
                            i32.const 0
                            i32.load offset=1052
                            local.tee 10
                            i32.eqz
                            br_if 1 (;@11;)
                            local.get 10
                            i32.const -1
                            i32.add
                            local.get 10
                            i32.const -1
                            i32.xor
                            i32.and
                            local.tee 4
                            local.get 4
                            i32.const 12
                            i32.shr_u
                            i32.const 16
                            i32.and
                            local.tee 4
                            i32.shr_u
                            local.tee 3
                            i32.const 5
                            i32.shr_u
                            i32.const 8
                            i32.and
                            local.tee 6
                            local.get 4
                            i32.or
                            local.get 3
                            local.get 6
                            i32.shr_u
                            local.tee 4
                            i32.const 2
                            i32.shr_u
                            i32.const 4
                            i32.and
                            local.tee 3
                            i32.or
                            local.get 4
                            local.get 3
                            i32.shr_u
                            local.tee 4
                            i32.const 1
                            i32.shr_u
                            i32.const 2
                            i32.and
                            local.tee 3
                            i32.or
                            local.get 4
                            local.get 3
                            i32.shr_u
                            local.tee 4
                            i32.const 1
                            i32.shr_u
                            i32.const 1
                            i32.and
                            local.tee 3
                            i32.or
                            local.get 4
                            local.get 3
                            i32.shr_u
                            i32.add
                            i32.const 2
                            i32.shl
                            i32.const 1352
                            i32.add
                            i32.load
                            local.tee 0
                            i32.load offset=4
                            i32.const -8
                            i32.and
                            local.get 5
                            i32.sub
                            local.set 3
                            local.get 0
                            local.set 6
                            block  ;; label = @13
                              loop  ;; label = @14
                                block  ;; label = @15
                                  local.get 6
                                  i32.load offset=16
                                  local.tee 4
                                  br_if 0 (;@15;)
                                  local.get 6
                                  i32.const 20
                                  i32.add
                                  i32.load
                                  local.tee 4
                                  i32.eqz
                                  br_if 2 (;@13;)
                                end
                                local.get 4
                                i32.load offset=4
                                i32.const -8
                                i32.and
                                local.get 5
                                i32.sub
                                local.tee 6
                                local.get 3
                                local.get 6
                                local.get 3
                                i32.lt_u
                                local.tee 6
                                select
                                local.set 3
                                local.get 4
                                local.get 0
                                local.get 6
                                select
                                local.set 0
                                local.get 4
                                local.set 6
                                br 0 (;@14;)
                              end
                            end
                            local.get 0
                            i32.load offset=24
                            local.set 11
                            block  ;; label = @13
                              local.get 0
                              i32.load offset=12
                              local.tee 9
                              local.get 0
                              i32.eq
                              br_if 0 (;@13;)
                              local.get 0
                              i32.load offset=8
                              local.tee 4
                              i32.const 0
                              i32.load offset=1064
                              i32.lt_u
                              drop
                              local.get 9
                              local.get 4
                              i32.store offset=8
                              local.get 4
                              local.get 9
                              i32.store offset=12
                              br 11 (;@2;)
                            end
                            block  ;; label = @13
                              local.get 0
                              i32.const 20
                              i32.add
                              local.tee 6
                              i32.load
                              local.tee 4
                              br_if 0 (;@13;)
                              local.get 0
                              i32.load offset=16
                              local.tee 4
                              i32.eqz
                              br_if 3 (;@10;)
                              local.get 0
                              i32.const 16
                              i32.add
                              local.set 6
                            end
                            loop  ;; label = @13
                              local.get 6
                              local.set 2
                              local.get 4
                              local.tee 9
                              i32.const 20
                              i32.add
                              local.tee 6
                              i32.load
                              local.tee 4
                              br_if 0 (;@13;)
                              local.get 9
                              i32.const 16
                              i32.add
                              local.set 6
                              local.get 9
                              i32.load offset=16
                              local.tee 4
                              br_if 0 (;@13;)
                            end
                            local.get 2
                            i32.const 0
                            i32.store
                            br 10 (;@2;)
                          end
                          i32.const -1
                          local.set 5
                          local.get 0
                          i32.const -65
                          i32.gt_u
                          br_if 0 (;@11;)
                          local.get 0
                          i32.const 19
                          i32.add
                          local.tee 4
                          i32.const -16
                          i32.and
                          local.set 5
                          i32.const 0
                          i32.load offset=1052
                          local.tee 10
                          i32.eqz
                          br_if 0 (;@11;)
                          i32.const 0
                          local.set 8
                          block  ;; label = @12
                            local.get 5
                            i32.const 256
                            i32.lt_u
                            br_if 0 (;@12;)
                            i32.const 31
                            local.set 8
                            local.get 5
                            i32.const 16777215
                            i32.gt_u
                            br_if 0 (;@12;)
                            local.get 4
                            i32.const 8
                            i32.shr_u
                            local.tee 4
                            local.get 4
                            i32.const 1048320
                            i32.add
                            i32.const 16
                            i32.shr_u
                            i32.const 8
                            i32.and
                            local.tee 4
                            i32.shl
                            local.tee 3
                            local.get 3
                            i32.const 520192
                            i32.add
                            i32.const 16
                            i32.shr_u
                            i32.const 4
                            i32.and
                            local.tee 3
                            i32.shl
                            local.tee 6
                            local.get 6
                            i32.const 245760
                            i32.add
                            i32.const 16
                            i32.shr_u
                            i32.const 2
                            i32.and
                            local.tee 6
                            i32.shl
                            i32.const 15
                            i32.shr_u
                            local.get 4
                            local.get 3
                            i32.or
                            local.get 6
                            i32.or
                            i32.sub
                            local.tee 4
                            i32.const 1
                            i32.shl
                            local.get 5
                            local.get 4
                            i32.const 21
                            i32.add
                            i32.shr_u
                            i32.const 1
                            i32.and
                            i32.or
                            i32.const 28
                            i32.add
                            local.set 8
                          end
                          i32.const 0
                          local.get 5
                          i32.sub
                          local.set 3
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  local.get 8
                                  i32.const 2
                                  i32.shl
                                  i32.const 1352
                                  i32.add
                                  i32.load
                                  local.tee 6
                                  br_if 0 (;@15;)
                                  i32.const 0
                                  local.set 4
                                  i32.const 0
                                  local.set 9
                                  br 1 (;@14;)
                                end
                                i32.const 0
                                local.set 4
                                local.get 5
                                i32.const 0
                                i32.const 25
                                local.get 8
                                i32.const 1
                                i32.shr_u
                                i32.sub
                                local.get 8
                                i32.const 31
                                i32.eq
                                select
                                i32.shl
                                local.set 0
                                i32.const 0
                                local.set 9
                                loop  ;; label = @15
                                  block  ;; label = @16
                                    local.get 6
                                    i32.load offset=4
                                    i32.const -8
                                    i32.and
                                    local.get 5
                                    i32.sub
                                    local.tee 7
                                    local.get 3
                                    i32.ge_u
                                    br_if 0 (;@16;)
                                    local.get 7
                                    local.set 3
                                    local.get 6
                                    local.set 9
                                    local.get 7
                                    br_if 0 (;@16;)
                                    i32.const 0
                                    local.set 3
                                    local.get 6
                                    local.set 9
                                    local.get 6
                                    local.set 4
                                    br 3 (;@13;)
                                  end
                                  local.get 4
                                  local.get 6
                                  i32.const 20
                                  i32.add
                                  i32.load
                                  local.tee 7
                                  local.get 7
                                  local.get 6
                                  local.get 0
                                  i32.const 29
                                  i32.shr_u
                                  i32.const 4
                                  i32.and
                                  i32.add
                                  i32.const 16
                                  i32.add
                                  i32.load
                                  local.tee 6
                                  i32.eq
                                  select
                                  local.get 4
                                  local.get 7
                                  select
                                  local.set 4
                                  local.get 0
                                  i32.const 1
                                  i32.shl
                                  local.set 0
                                  local.get 6
                                  br_if 0 (;@15;)
                                end
                              end
                              block  ;; label = @14
                                local.get 4
                                local.get 9
                                i32.or
                                br_if 0 (;@14;)
                                i32.const 0
                                local.set 9
                                i32.const 2
                                local.get 8
                                i32.shl
                                local.tee 4
                                i32.const 0
                                local.get 4
                                i32.sub
                                i32.or
                                local.get 10
                                i32.and
                                local.tee 4
                                i32.eqz
                                br_if 3 (;@11;)
                                local.get 4
                                i32.const -1
                                i32.add
                                local.get 4
                                i32.const -1
                                i32.xor
                                i32.and
                                local.tee 4
                                local.get 4
                                i32.const 12
                                i32.shr_u
                                i32.const 16
                                i32.and
                                local.tee 4
                                i32.shr_u
                                local.tee 6
                                i32.const 5
                                i32.shr_u
                                i32.const 8
                                i32.and
                                local.tee 0
                                local.get 4
                                i32.or
                                local.get 6
                                local.get 0
                                i32.shr_u
                                local.tee 4
                                i32.const 2
                                i32.shr_u
                                i32.const 4
                                i32.and
                                local.tee 6
                                i32.or
                                local.get 4
                                local.get 6
                                i32.shr_u
                                local.tee 4
                                i32.const 1
                                i32.shr_u
                                i32.const 2
                                i32.and
                                local.tee 6
                                i32.or
                                local.get 4
                                local.get 6
                                i32.shr_u
                                local.tee 4
                                i32.const 1
                                i32.shr_u
                                i32.const 1
                                i32.and
                                local.tee 6
                                i32.or
                                local.get 4
                                local.get 6
                                i32.shr_u
                                i32.add
                                i32.const 2
                                i32.shl
                                i32.const 1352
                                i32.add
                                i32.load
                                local.set 4
                              end
                              local.get 4
                              i32.eqz
                              br_if 1 (;@12;)
                            end
                            loop  ;; label = @13
                              local.get 4
                              i32.load offset=4
                              i32.const -8
                              i32.and
                              local.get 5
                              i32.sub
                              local.tee 7
                              local.get 3
                              i32.lt_u
                              local.set 0
                              block  ;; label = @14
                                local.get 4
                                i32.load offset=16
                                local.tee 6
                                br_if 0 (;@14;)
                                local.get 4
                                i32.const 20
                                i32.add
                                i32.load
                                local.set 6
                              end
                              local.get 7
                              local.get 3
                              local.get 0
                              select
                              local.set 3
                              local.get 4
                              local.get 9
                              local.get 0
                              select
                              local.set 9
                              local.get 6
                              local.set 4
                              local.get 6
                              br_if 0 (;@13;)
                            end
                          end
                          local.get 9
                          i32.eqz
                          br_if 0 (;@11;)
                          local.get 3
                          i32.const 0
                          i32.load offset=1056
                          local.get 5
                          i32.sub
                          i32.ge_u
                          br_if 0 (;@11;)
                          local.get 9
                          i32.load offset=24
                          local.set 2
                          block  ;; label = @12
                            local.get 9
                            i32.load offset=12
                            local.tee 0
                            local.get 9
                            i32.eq
                            br_if 0 (;@12;)
                            local.get 9
                            i32.load offset=8
                            local.tee 4
                            i32.const 0
                            i32.load offset=1064
                            i32.lt_u
                            drop
                            local.get 0
                            local.get 4
                            i32.store offset=8
                            local.get 4
                            local.get 0
                            i32.store offset=12
                            br 9 (;@3;)
                          end
                          block  ;; label = @12
                            local.get 9
                            i32.const 20
                            i32.add
                            local.tee 6
                            i32.load
                            local.tee 4
                            br_if 0 (;@12;)
                            local.get 9
                            i32.load offset=16
                            local.tee 4
                            i32.eqz
                            br_if 3 (;@9;)
                            local.get 9
                            i32.const 16
                            i32.add
                            local.set 6
                          end
                          loop  ;; label = @12
                            local.get 6
                            local.set 7
                            local.get 4
                            local.tee 0
                            i32.const 20
                            i32.add
                            local.tee 6
                            i32.load
                            local.tee 4
                            br_if 0 (;@12;)
                            local.get 0
                            i32.const 16
                            i32.add
                            local.set 6
                            local.get 0
                            i32.load offset=16
                            local.tee 4
                            br_if 0 (;@12;)
                          end
                          local.get 7
                          i32.const 0
                          i32.store
                          br 8 (;@3;)
                        end
                        block  ;; label = @11
                          i32.const 0
                          i32.load offset=1056
                          local.tee 4
                          local.get 5
                          i32.lt_u
                          br_if 0 (;@11;)
                          i32.const 0
                          i32.load offset=1068
                          local.set 3
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 4
                              local.get 5
                              i32.sub
                              local.tee 6
                              i32.const 16
                              i32.lt_u
                              br_if 0 (;@13;)
                              local.get 3
                              local.get 5
                              i32.add
                              local.tee 0
                              local.get 6
                              i32.const 1
                              i32.or
                              i32.store offset=4
                              local.get 3
                              local.get 4
                              i32.add
                              local.get 6
                              i32.store
                              local.get 3
                              local.get 5
                              i32.const 3
                              i32.or
                              i32.store offset=4
                              br 1 (;@12;)
                            end
                            local.get 3
                            local.get 4
                            i32.const 3
                            i32.or
                            i32.store offset=4
                            local.get 3
                            local.get 4
                            i32.add
                            local.tee 4
                            local.get 4
                            i32.load offset=4
                            i32.const 1
                            i32.or
                            i32.store offset=4
                            i32.const 0
                            local.set 0
                            i32.const 0
                            local.set 6
                          end
                          i32.const 0
                          local.get 6
                          i32.store offset=1056
                          i32.const 0
                          local.get 0
                          i32.store offset=1068
                          local.get 3
                          i32.const 8
                          i32.add
                          local.set 4
                          br 10 (;@1;)
                        end
                        block  ;; label = @11
                          i32.const 0
                          i32.load offset=1060
                          local.tee 6
                          local.get 5
                          i32.le_u
                          br_if 0 (;@11;)
                          local.get 2
                          local.get 5
                          i32.add
                          local.tee 4
                          local.get 6
                          local.get 5
                          i32.sub
                          local.tee 3
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          i32.const 0
                          local.get 4
                          i32.store offset=1072
                          i32.const 0
                          local.get 3
                          i32.store offset=1060
                          local.get 2
                          local.get 5
                          i32.const 3
                          i32.or
                          i32.store offset=4
                          local.get 2
                          i32.const 8
                          i32.add
                          local.set 4
                          br 10 (;@1;)
                        end
                        block  ;; label = @11
                          block  ;; label = @12
                            i32.const 0
                            i32.load offset=1520
                            i32.eqz
                            br_if 0 (;@12;)
                            i32.const 0
                            i32.load offset=1528
                            local.set 3
                            br 1 (;@11;)
                          end
                          i32.const 0
                          i64.const -1
                          i64.store offset=1532 align=4
                          i32.const 0
                          i64.const 281474976776192
                          i64.store offset=1524 align=4
                          i32.const 0
                          local.get 1
                          i32.const 12
                          i32.add
                          i32.const -16
                          i32.and
                          i32.const 1431655768
                          i32.xor
                          i32.store offset=1520
                          i32.const 0
                          i32.const 0
                          i32.store offset=1540
                          i32.const 0
                          i32.const 0
                          i32.store offset=1492
                          i32.const 65536
                          local.set 3
                        end
                        i32.const 0
                        local.set 4
                        block  ;; label = @11
                          local.get 3
                          local.get 5
                          i32.const 71
                          i32.add
                          local.tee 8
                          i32.add
                          local.tee 0
                          i32.const 0
                          local.get 3
                          i32.sub
                          local.tee 7
                          i32.and
                          local.tee 9
                          local.get 5
                          i32.gt_u
                          br_if 0 (;@11;)
                          i32.const 0
                          i32.const 48
                          i32.store offset=1544
                          br 10 (;@1;)
                        end
                        block  ;; label = @11
                          i32.const 0
                          i32.load offset=1488
                          local.tee 4
                          i32.eqz
                          br_if 0 (;@11;)
                          block  ;; label = @12
                            i32.const 0
                            i32.load offset=1480
                            local.tee 3
                            local.get 9
                            i32.add
                            local.tee 10
                            local.get 3
                            i32.le_u
                            br_if 0 (;@12;)
                            local.get 10
                            local.get 4
                            i32.le_u
                            br_if 1 (;@11;)
                          end
                          i32.const 0
                          local.set 4
                          i32.const 0
                          i32.const 48
                          i32.store offset=1544
                          br 10 (;@1;)
                        end
                        i32.const 0
                        i32.load8_u offset=1492
                        i32.const 4
                        i32.and
                        br_if 4 (;@6;)
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 2
                              i32.eqz
                              br_if 0 (;@13;)
                              i32.const 1496
                              local.set 4
                              loop  ;; label = @14
                                block  ;; label = @15
                                  local.get 4
                                  i32.load
                                  local.tee 3
                                  local.get 2
                                  i32.gt_u
                                  br_if 0 (;@15;)
                                  local.get 3
                                  local.get 4
                                  i32.load offset=4
                                  i32.add
                                  local.get 2
                                  i32.gt_u
                                  br_if 3 (;@12;)
                                end
                                local.get 4
                                i32.load offset=8
                                local.tee 4
                                br_if 0 (;@14;)
                              end
                            end
                            i32.const 0
                            call 8
                            local.tee 0
                            i32.const -1
                            i32.eq
                            br_if 5 (;@7;)
                            local.get 9
                            local.set 7
                            block  ;; label = @13
                              i32.const 0
                              i32.load offset=1524
                              local.tee 4
                              i32.const -1
                              i32.add
                              local.tee 3
                              local.get 0
                              i32.and
                              i32.eqz
                              br_if 0 (;@13;)
                              local.get 9
                              local.get 0
                              i32.sub
                              local.get 3
                              local.get 0
                              i32.add
                              i32.const 0
                              local.get 4
                              i32.sub
                              i32.and
                              i32.add
                              local.set 7
                            end
                            local.get 7
                            local.get 5
                            i32.le_u
                            br_if 5 (;@7;)
                            local.get 7
                            i32.const 2147483646
                            i32.gt_u
                            br_if 5 (;@7;)
                            block  ;; label = @13
                              i32.const 0
                              i32.load offset=1488
                              local.tee 4
                              i32.eqz
                              br_if 0 (;@13;)
                              i32.const 0
                              i32.load offset=1480
                              local.tee 3
                              local.get 7
                              i32.add
                              local.tee 6
                              local.get 3
                              i32.le_u
                              br_if 6 (;@7;)
                              local.get 6
                              local.get 4
                              i32.gt_u
                              br_if 6 (;@7;)
                            end
                            local.get 7
                            call 8
                            local.tee 4
                            local.get 0
                            i32.ne
                            br_if 1 (;@11;)
                            br 7 (;@5;)
                          end
                          local.get 0
                          local.get 6
                          i32.sub
                          local.get 7
                          i32.and
                          local.tee 7
                          i32.const 2147483646
                          i32.gt_u
                          br_if 4 (;@7;)
                          local.get 7
                          call 8
                          local.tee 0
                          local.get 4
                          i32.load
                          local.get 4
                          i32.load offset=4
                          i32.add
                          i32.eq
                          br_if 3 (;@8;)
                          local.get 0
                          local.set 4
                        end
                        block  ;; label = @11
                          local.get 4
                          i32.const -1
                          i32.eq
                          br_if 0 (;@11;)
                          local.get 5
                          i32.const 72
                          i32.add
                          local.get 7
                          i32.le_u
                          br_if 0 (;@11;)
                          block  ;; label = @12
                            local.get 8
                            local.get 7
                            i32.sub
                            i32.const 0
                            i32.load offset=1528
                            local.tee 3
                            i32.add
                            i32.const 0
                            local.get 3
                            i32.sub
                            i32.and
                            local.tee 3
                            i32.const 2147483646
                            i32.le_u
                            br_if 0 (;@12;)
                            local.get 4
                            local.set 0
                            br 7 (;@5;)
                          end
                          block  ;; label = @12
                            local.get 3
                            call 8
                            i32.const -1
                            i32.eq
                            br_if 0 (;@12;)
                            local.get 3
                            local.get 7
                            i32.add
                            local.set 7
                            local.get 4
                            local.set 0
                            br 7 (;@5;)
                          end
                          i32.const 0
                          local.get 7
                          i32.sub
                          call 8
                          drop
                          br 4 (;@7;)
                        end
                        local.get 4
                        local.set 0
                        local.get 4
                        i32.const -1
                        i32.ne
                        br_if 5 (;@5;)
                        br 3 (;@7;)
                      end
                      i32.const 0
                      local.set 9
                      br 7 (;@2;)
                    end
                    i32.const 0
                    local.set 0
                    br 5 (;@3;)
                  end
                  local.get 0
                  i32.const -1
                  i32.ne
                  br_if 2 (;@5;)
                end
                i32.const 0
                i32.const 0
                i32.load offset=1492
                i32.const 4
                i32.or
                i32.store offset=1492
              end
              local.get 9
              i32.const 2147483646
              i32.gt_u
              br_if 1 (;@4;)
              local.get 9
              call 8
              local.set 0
              i32.const 0
              call 8
              local.set 4
              local.get 0
              i32.const -1
              i32.eq
              br_if 1 (;@4;)
              local.get 4
              i32.const -1
              i32.eq
              br_if 1 (;@4;)
              local.get 0
              local.get 4
              i32.ge_u
              br_if 1 (;@4;)
              local.get 4
              local.get 0
              i32.sub
              local.tee 7
              local.get 5
              i32.const 56
              i32.add
              i32.le_u
              br_if 1 (;@4;)
            end
            i32.const 0
            i32.const 0
            i32.load offset=1480
            local.get 7
            i32.add
            local.tee 4
            i32.store offset=1480
            block  ;; label = @5
              local.get 4
              i32.const 0
              i32.load offset=1484
              i32.le_u
              br_if 0 (;@5;)
              i32.const 0
              local.get 4
              i32.store offset=1484
            end
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    i32.const 0
                    i32.load offset=1072
                    local.tee 3
                    i32.eqz
                    br_if 0 (;@8;)
                    i32.const 1496
                    local.set 4
                    loop  ;; label = @9
                      local.get 0
                      local.get 4
                      i32.load
                      local.tee 6
                      local.get 4
                      i32.load offset=4
                      local.tee 9
                      i32.add
                      i32.eq
                      br_if 2 (;@7;)
                      local.get 4
                      i32.load offset=8
                      local.tee 4
                      br_if 0 (;@9;)
                      br 3 (;@6;)
                    end
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      i32.const 0
                      i32.load offset=1064
                      local.tee 4
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 0
                      local.get 4
                      i32.ge_u
                      br_if 1 (;@8;)
                    end
                    i32.const 0
                    local.get 0
                    i32.store offset=1064
                  end
                  i32.const 0
                  local.set 4
                  i32.const 0
                  local.get 7
                  i32.store offset=1500
                  i32.const 0
                  local.get 0
                  i32.store offset=1496
                  i32.const 0
                  i32.const -1
                  i32.store offset=1080
                  i32.const 0
                  i32.const 0
                  i32.load offset=1520
                  i32.store offset=1084
                  i32.const 0
                  i32.const 0
                  i32.store offset=1508
                  loop  ;; label = @8
                    local.get 4
                    i32.const 1108
                    i32.add
                    local.get 4
                    i32.const 1096
                    i32.add
                    local.tee 3
                    i32.store
                    local.get 3
                    local.get 4
                    i32.const 1088
                    i32.add
                    local.tee 6
                    i32.store
                    local.get 4
                    i32.const 1100
                    i32.add
                    local.get 6
                    i32.store
                    local.get 4
                    i32.const 1116
                    i32.add
                    local.get 4
                    i32.const 1104
                    i32.add
                    local.tee 6
                    i32.store
                    local.get 6
                    local.get 3
                    i32.store
                    local.get 4
                    i32.const 1124
                    i32.add
                    local.get 4
                    i32.const 1112
                    i32.add
                    local.tee 3
                    i32.store
                    local.get 3
                    local.get 6
                    i32.store
                    local.get 4
                    i32.const 1120
                    i32.add
                    local.get 3
                    i32.store
                    local.get 4
                    i32.const 32
                    i32.add
                    local.tee 4
                    i32.const 256
                    i32.ne
                    br_if 0 (;@8;)
                  end
                  local.get 0
                  i32.const -8
                  local.get 0
                  i32.sub
                  i32.const 15
                  i32.and
                  i32.const 0
                  local.get 0
                  i32.const 8
                  i32.add
                  i32.const 15
                  i32.and
                  select
                  local.tee 4
                  i32.add
                  local.tee 3
                  local.get 7
                  i32.const -56
                  i32.add
                  local.tee 6
                  local.get 4
                  i32.sub
                  local.tee 4
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  i32.const 0
                  i32.const 0
                  i32.load offset=1536
                  i32.store offset=1076
                  i32.const 0
                  local.get 4
                  i32.store offset=1060
                  i32.const 0
                  local.get 3
                  i32.store offset=1072
                  local.get 0
                  local.get 6
                  i32.add
                  i32.const 56
                  i32.store offset=4
                  br 2 (;@5;)
                end
                local.get 4
                i32.load8_u offset=12
                i32.const 8
                i32.and
                br_if 0 (;@6;)
                local.get 3
                local.get 6
                i32.lt_u
                br_if 0 (;@6;)
                local.get 3
                local.get 0
                i32.ge_u
                br_if 0 (;@6;)
                local.get 3
                i32.const -8
                local.get 3
                i32.sub
                i32.const 15
                i32.and
                i32.const 0
                local.get 3
                i32.const 8
                i32.add
                i32.const 15
                i32.and
                select
                local.tee 6
                i32.add
                local.tee 0
                i32.const 0
                i32.load offset=1060
                local.get 7
                i32.add
                local.tee 2
                local.get 6
                i32.sub
                local.tee 6
                i32.const 1
                i32.or
                i32.store offset=4
                local.get 4
                local.get 9
                local.get 7
                i32.add
                i32.store offset=4
                i32.const 0
                i32.const 0
                i32.load offset=1536
                i32.store offset=1076
                i32.const 0
                local.get 6
                i32.store offset=1060
                i32.const 0
                local.get 0
                i32.store offset=1072
                local.get 3
                local.get 2
                i32.add
                i32.const 56
                i32.store offset=4
                br 1 (;@5;)
              end
              block  ;; label = @6
                local.get 0
                i32.const 0
                i32.load offset=1064
                local.tee 9
                i32.ge_u
                br_if 0 (;@6;)
                i32.const 0
                local.get 0
                i32.store offset=1064
                local.get 0
                local.set 9
              end
              local.get 0
              local.get 7
              i32.add
              local.set 6
              i32.const 1496
              local.set 4
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            loop  ;; label = @13
                              local.get 4
                              i32.load
                              local.get 6
                              i32.eq
                              br_if 1 (;@12;)
                              local.get 4
                              i32.load offset=8
                              local.tee 4
                              br_if 0 (;@13;)
                              br 2 (;@11;)
                            end
                          end
                          local.get 4
                          i32.load8_u offset=12
                          i32.const 8
                          i32.and
                          i32.eqz
                          br_if 1 (;@10;)
                        end
                        i32.const 1496
                        local.set 4
                        loop  ;; label = @11
                          block  ;; label = @12
                            local.get 4
                            i32.load
                            local.tee 6
                            local.get 3
                            i32.gt_u
                            br_if 0 (;@12;)
                            local.get 6
                            local.get 4
                            i32.load offset=4
                            i32.add
                            local.tee 6
                            local.get 3
                            i32.gt_u
                            br_if 3 (;@9;)
                          end
                          local.get 4
                          i32.load offset=8
                          local.set 4
                          br 0 (;@11;)
                        end
                      end
                      local.get 4
                      local.get 0
                      i32.store
                      local.get 4
                      local.get 4
                      i32.load offset=4
                      local.get 7
                      i32.add
                      i32.store offset=4
                      local.get 0
                      i32.const -8
                      local.get 0
                      i32.sub
                      i32.const 15
                      i32.and
                      i32.const 0
                      local.get 0
                      i32.const 8
                      i32.add
                      i32.const 15
                      i32.and
                      select
                      i32.add
                      local.tee 2
                      local.get 5
                      i32.const 3
                      i32.or
                      i32.store offset=4
                      local.get 6
                      i32.const -8
                      local.get 6
                      i32.sub
                      i32.const 15
                      i32.and
                      i32.const 0
                      local.get 6
                      i32.const 8
                      i32.add
                      i32.const 15
                      i32.and
                      select
                      i32.add
                      local.tee 7
                      local.get 2
                      local.get 5
                      i32.add
                      local.tee 5
                      i32.sub
                      local.set 4
                      block  ;; label = @10
                        local.get 7
                        local.get 3
                        i32.ne
                        br_if 0 (;@10;)
                        i32.const 0
                        local.get 5
                        i32.store offset=1072
                        i32.const 0
                        i32.const 0
                        i32.load offset=1060
                        local.get 4
                        i32.add
                        local.tee 4
                        i32.store offset=1060
                        local.get 5
                        local.get 4
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        br 3 (;@7;)
                      end
                      block  ;; label = @10
                        local.get 7
                        i32.const 0
                        i32.load offset=1068
                        i32.ne
                        br_if 0 (;@10;)
                        i32.const 0
                        local.get 5
                        i32.store offset=1068
                        i32.const 0
                        i32.const 0
                        i32.load offset=1056
                        local.get 4
                        i32.add
                        local.tee 4
                        i32.store offset=1056
                        local.get 5
                        local.get 4
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        local.get 5
                        local.get 4
                        i32.add
                        local.get 4
                        i32.store
                        br 3 (;@7;)
                      end
                      block  ;; label = @10
                        local.get 7
                        i32.load offset=4
                        local.tee 3
                        i32.const 3
                        i32.and
                        i32.const 1
                        i32.ne
                        br_if 0 (;@10;)
                        local.get 3
                        i32.const -8
                        i32.and
                        local.set 8
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 3
                            i32.const 255
                            i32.gt_u
                            br_if 0 (;@12;)
                            local.get 7
                            i32.load offset=8
                            local.tee 6
                            local.get 3
                            i32.const 3
                            i32.shr_u
                            local.tee 9
                            i32.const 3
                            i32.shl
                            i32.const 1088
                            i32.add
                            local.tee 0
                            i32.eq
                            drop
                            block  ;; label = @13
                              local.get 7
                              i32.load offset=12
                              local.tee 3
                              local.get 6
                              i32.ne
                              br_if 0 (;@13;)
                              i32.const 0
                              i32.const 0
                              i32.load offset=1048
                              i32.const -2
                              local.get 9
                              i32.rotl
                              i32.and
                              i32.store offset=1048
                              br 2 (;@11;)
                            end
                            local.get 3
                            local.get 0
                            i32.eq
                            drop
                            local.get 3
                            local.get 6
                            i32.store offset=8
                            local.get 6
                            local.get 3
                            i32.store offset=12
                            br 1 (;@11;)
                          end
                          local.get 7
                          i32.load offset=24
                          local.set 10
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 7
                              i32.load offset=12
                              local.tee 0
                              local.get 7
                              i32.eq
                              br_if 0 (;@13;)
                              local.get 7
                              i32.load offset=8
                              local.tee 3
                              local.get 9
                              i32.lt_u
                              drop
                              local.get 0
                              local.get 3
                              i32.store offset=8
                              local.get 3
                              local.get 0
                              i32.store offset=12
                              br 1 (;@12;)
                            end
                            block  ;; label = @13
                              local.get 7
                              i32.const 20
                              i32.add
                              local.tee 3
                              i32.load
                              local.tee 6
                              br_if 0 (;@13;)
                              local.get 7
                              i32.const 16
                              i32.add
                              local.tee 3
                              i32.load
                              local.tee 6
                              br_if 0 (;@13;)
                              i32.const 0
                              local.set 0
                              br 1 (;@12;)
                            end
                            loop  ;; label = @13
                              local.get 3
                              local.set 9
                              local.get 6
                              local.tee 0
                              i32.const 20
                              i32.add
                              local.tee 3
                              i32.load
                              local.tee 6
                              br_if 0 (;@13;)
                              local.get 0
                              i32.const 16
                              i32.add
                              local.set 3
                              local.get 0
                              i32.load offset=16
                              local.tee 6
                              br_if 0 (;@13;)
                            end
                            local.get 9
                            i32.const 0
                            i32.store
                          end
                          local.get 10
                          i32.eqz
                          br_if 0 (;@11;)
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 7
                              local.get 7
                              i32.load offset=28
                              local.tee 6
                              i32.const 2
                              i32.shl
                              i32.const 1352
                              i32.add
                              local.tee 3
                              i32.load
                              i32.ne
                              br_if 0 (;@13;)
                              local.get 3
                              local.get 0
                              i32.store
                              local.get 0
                              br_if 1 (;@12;)
                              i32.const 0
                              i32.const 0
                              i32.load offset=1052
                              i32.const -2
                              local.get 6
                              i32.rotl
                              i32.and
                              i32.store offset=1052
                              br 2 (;@11;)
                            end
                            local.get 10
                            i32.const 16
                            i32.const 20
                            local.get 10
                            i32.load offset=16
                            local.get 7
                            i32.eq
                            select
                            i32.add
                            local.get 0
                            i32.store
                            local.get 0
                            i32.eqz
                            br_if 1 (;@11;)
                          end
                          local.get 0
                          local.get 10
                          i32.store offset=24
                          block  ;; label = @12
                            local.get 7
                            i32.load offset=16
                            local.tee 3
                            i32.eqz
                            br_if 0 (;@12;)
                            local.get 0
                            local.get 3
                            i32.store offset=16
                            local.get 3
                            local.get 0
                            i32.store offset=24
                          end
                          local.get 7
                          i32.load offset=20
                          local.tee 3
                          i32.eqz
                          br_if 0 (;@11;)
                          local.get 0
                          i32.const 20
                          i32.add
                          local.get 3
                          i32.store
                          local.get 3
                          local.get 0
                          i32.store offset=24
                        end
                        local.get 8
                        local.get 4
                        i32.add
                        local.set 4
                        local.get 7
                        local.get 8
                        i32.add
                        local.tee 7
                        i32.load offset=4
                        local.set 3
                      end
                      local.get 7
                      local.get 3
                      i32.const -2
                      i32.and
                      i32.store offset=4
                      local.get 5
                      local.get 4
                      i32.add
                      local.get 4
                      i32.store
                      local.get 5
                      local.get 4
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      block  ;; label = @10
                        local.get 4
                        i32.const 255
                        i32.gt_u
                        br_if 0 (;@10;)
                        local.get 4
                        i32.const -8
                        i32.and
                        i32.const 1088
                        i32.add
                        local.set 3
                        block  ;; label = @11
                          block  ;; label = @12
                            i32.const 0
                            i32.load offset=1048
                            local.tee 6
                            i32.const 1
                            local.get 4
                            i32.const 3
                            i32.shr_u
                            i32.shl
                            local.tee 4
                            i32.and
                            br_if 0 (;@12;)
                            i32.const 0
                            local.get 6
                            local.get 4
                            i32.or
                            i32.store offset=1048
                            local.get 3
                            local.set 4
                            br 1 (;@11;)
                          end
                          local.get 3
                          i32.load offset=8
                          local.set 4
                        end
                        local.get 4
                        local.get 5
                        i32.store offset=12
                        local.get 3
                        local.get 5
                        i32.store offset=8
                        local.get 5
                        local.get 3
                        i32.store offset=12
                        local.get 5
                        local.get 4
                        i32.store offset=8
                        br 3 (;@7;)
                      end
                      i32.const 31
                      local.set 3
                      block  ;; label = @10
                        local.get 4
                        i32.const 16777215
                        i32.gt_u
                        br_if 0 (;@10;)
                        local.get 4
                        i32.const 8
                        i32.shr_u
                        local.tee 3
                        local.get 3
                        i32.const 1048320
                        i32.add
                        i32.const 16
                        i32.shr_u
                        i32.const 8
                        i32.and
                        local.tee 3
                        i32.shl
                        local.tee 6
                        local.get 6
                        i32.const 520192
                        i32.add
                        i32.const 16
                        i32.shr_u
                        i32.const 4
                        i32.and
                        local.tee 6
                        i32.shl
                        local.tee 0
                        local.get 0
                        i32.const 245760
                        i32.add
                        i32.const 16
                        i32.shr_u
                        i32.const 2
                        i32.and
                        local.tee 0
                        i32.shl
                        i32.const 15
                        i32.shr_u
                        local.get 3
                        local.get 6
                        i32.or
                        local.get 0
                        i32.or
                        i32.sub
                        local.tee 3
                        i32.const 1
                        i32.shl
                        local.get 4
                        local.get 3
                        i32.const 21
                        i32.add
                        i32.shr_u
                        i32.const 1
                        i32.and
                        i32.or
                        i32.const 28
                        i32.add
                        local.set 3
                      end
                      local.get 5
                      local.get 3
                      i32.store offset=28
                      local.get 5
                      i64.const 0
                      i64.store offset=16 align=4
                      local.get 3
                      i32.const 2
                      i32.shl
                      i32.const 1352
                      i32.add
                      local.set 6
                      block  ;; label = @10
                        i32.const 0
                        i32.load offset=1052
                        local.tee 0
                        i32.const 1
                        local.get 3
                        i32.shl
                        local.tee 9
                        i32.and
                        br_if 0 (;@10;)
                        local.get 6
                        local.get 5
                        i32.store
                        i32.const 0
                        local.get 0
                        local.get 9
                        i32.or
                        i32.store offset=1052
                        local.get 5
                        local.get 6
                        i32.store offset=24
                        local.get 5
                        local.get 5
                        i32.store offset=8
                        local.get 5
                        local.get 5
                        i32.store offset=12
                        br 3 (;@7;)
                      end
                      local.get 4
                      i32.const 0
                      i32.const 25
                      local.get 3
                      i32.const 1
                      i32.shr_u
                      i32.sub
                      local.get 3
                      i32.const 31
                      i32.eq
                      select
                      i32.shl
                      local.set 3
                      local.get 6
                      i32.load
                      local.set 0
                      loop  ;; label = @10
                        local.get 0
                        local.tee 6
                        i32.load offset=4
                        i32.const -8
                        i32.and
                        local.get 4
                        i32.eq
                        br_if 2 (;@8;)
                        local.get 3
                        i32.const 29
                        i32.shr_u
                        local.set 0
                        local.get 3
                        i32.const 1
                        i32.shl
                        local.set 3
                        local.get 6
                        local.get 0
                        i32.const 4
                        i32.and
                        i32.add
                        i32.const 16
                        i32.add
                        local.tee 9
                        i32.load
                        local.tee 0
                        br_if 0 (;@10;)
                      end
                      local.get 9
                      local.get 5
                      i32.store
                      local.get 5
                      local.get 6
                      i32.store offset=24
                      local.get 5
                      local.get 5
                      i32.store offset=12
                      local.get 5
                      local.get 5
                      i32.store offset=8
                      br 2 (;@7;)
                    end
                    local.get 0
                    i32.const -8
                    local.get 0
                    i32.sub
                    i32.const 15
                    i32.and
                    i32.const 0
                    local.get 0
                    i32.const 8
                    i32.add
                    i32.const 15
                    i32.and
                    select
                    local.tee 4
                    i32.add
                    local.tee 2
                    local.get 7
                    i32.const -56
                    i32.add
                    local.tee 9
                    local.get 4
                    i32.sub
                    local.tee 4
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    local.get 0
                    local.get 9
                    i32.add
                    i32.const 56
                    i32.store offset=4
                    local.get 3
                    local.get 6
                    i32.const 55
                    local.get 6
                    i32.sub
                    i32.const 15
                    i32.and
                    i32.const 0
                    local.get 6
                    i32.const -55
                    i32.add
                    i32.const 15
                    i32.and
                    select
                    i32.add
                    i32.const -63
                    i32.add
                    local.tee 9
                    local.get 9
                    local.get 3
                    i32.const 16
                    i32.add
                    i32.lt_u
                    select
                    local.tee 9
                    i32.const 35
                    i32.store offset=4
                    i32.const 0
                    i32.const 0
                    i32.load offset=1536
                    i32.store offset=1076
                    i32.const 0
                    local.get 4
                    i32.store offset=1060
                    i32.const 0
                    local.get 2
                    i32.store offset=1072
                    local.get 9
                    i32.const 16
                    i32.add
                    i32.const 0
                    i64.load offset=1504 align=4
                    i64.store align=4
                    local.get 9
                    i32.const 0
                    i64.load offset=1496 align=4
                    i64.store offset=8 align=4
                    i32.const 0
                    local.get 9
                    i32.const 8
                    i32.add
                    i32.store offset=1504
                    i32.const 0
                    local.get 7
                    i32.store offset=1500
                    i32.const 0
                    local.get 0
                    i32.store offset=1496
                    i32.const 0
                    i32.const 0
                    i32.store offset=1508
                    local.get 9
                    i32.const 36
                    i32.add
                    local.set 4
                    loop  ;; label = @9
                      local.get 4
                      i32.const 7
                      i32.store
                      local.get 4
                      i32.const 4
                      i32.add
                      local.tee 4
                      local.get 6
                      i32.lt_u
                      br_if 0 (;@9;)
                    end
                    local.get 9
                    local.get 3
                    i32.eq
                    br_if 3 (;@5;)
                    local.get 9
                    local.get 9
                    i32.load offset=4
                    i32.const -2
                    i32.and
                    i32.store offset=4
                    local.get 9
                    local.get 9
                    local.get 3
                    i32.sub
                    local.tee 0
                    i32.store
                    local.get 3
                    local.get 0
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    block  ;; label = @9
                      local.get 0
                      i32.const 255
                      i32.gt_u
                      br_if 0 (;@9;)
                      local.get 0
                      i32.const -8
                      i32.and
                      i32.const 1088
                      i32.add
                      local.set 4
                      block  ;; label = @10
                        block  ;; label = @11
                          i32.const 0
                          i32.load offset=1048
                          local.tee 6
                          i32.const 1
                          local.get 0
                          i32.const 3
                          i32.shr_u
                          i32.shl
                          local.tee 0
                          i32.and
                          br_if 0 (;@11;)
                          i32.const 0
                          local.get 6
                          local.get 0
                          i32.or
                          i32.store offset=1048
                          local.get 4
                          local.set 6
                          br 1 (;@10;)
                        end
                        local.get 4
                        i32.load offset=8
                        local.set 6
                      end
                      local.get 6
                      local.get 3
                      i32.store offset=12
                      local.get 4
                      local.get 3
                      i32.store offset=8
                      local.get 3
                      local.get 4
                      i32.store offset=12
                      local.get 3
                      local.get 6
                      i32.store offset=8
                      br 4 (;@5;)
                    end
                    i32.const 31
                    local.set 4
                    block  ;; label = @9
                      local.get 0
                      i32.const 16777215
                      i32.gt_u
                      br_if 0 (;@9;)
                      local.get 0
                      i32.const 8
                      i32.shr_u
                      local.tee 4
                      local.get 4
                      i32.const 1048320
                      i32.add
                      i32.const 16
                      i32.shr_u
                      i32.const 8
                      i32.and
                      local.tee 4
                      i32.shl
                      local.tee 6
                      local.get 6
                      i32.const 520192
                      i32.add
                      i32.const 16
                      i32.shr_u
                      i32.const 4
                      i32.and
                      local.tee 6
                      i32.shl
                      local.tee 9
                      local.get 9
                      i32.const 245760
                      i32.add
                      i32.const 16
                      i32.shr_u
                      i32.const 2
                      i32.and
                      local.tee 9
                      i32.shl
                      i32.const 15
                      i32.shr_u
                      local.get 4
                      local.get 6
                      i32.or
                      local.get 9
                      i32.or
                      i32.sub
                      local.tee 4
                      i32.const 1
                      i32.shl
                      local.get 0
                      local.get 4
                      i32.const 21
                      i32.add
                      i32.shr_u
                      i32.const 1
                      i32.and
                      i32.or
                      i32.const 28
                      i32.add
                      local.set 4
                    end
                    local.get 3
                    local.get 4
                    i32.store offset=28
                    local.get 3
                    i64.const 0
                    i64.store offset=16 align=4
                    local.get 4
                    i32.const 2
                    i32.shl
                    i32.const 1352
                    i32.add
                    local.set 6
                    block  ;; label = @9
                      i32.const 0
                      i32.load offset=1052
                      local.tee 9
                      i32.const 1
                      local.get 4
                      i32.shl
                      local.tee 7
                      i32.and
                      br_if 0 (;@9;)
                      local.get 6
                      local.get 3
                      i32.store
                      i32.const 0
                      local.get 9
                      local.get 7
                      i32.or
                      i32.store offset=1052
                      local.get 3
                      local.get 6
                      i32.store offset=24
                      local.get 3
                      local.get 3
                      i32.store offset=8
                      local.get 3
                      local.get 3
                      i32.store offset=12
                      br 4 (;@5;)
                    end
                    local.get 0
                    i32.const 0
                    i32.const 25
                    local.get 4
                    i32.const 1
                    i32.shr_u
                    i32.sub
                    local.get 4
                    i32.const 31
                    i32.eq
                    select
                    i32.shl
                    local.set 4
                    local.get 6
                    i32.load
                    local.set 9
                    loop  ;; label = @9
                      local.get 9
                      local.tee 6
                      i32.load offset=4
                      i32.const -8
                      i32.and
                      local.get 0
                      i32.eq
                      br_if 3 (;@6;)
                      local.get 4
                      i32.const 29
                      i32.shr_u
                      local.set 9
                      local.get 4
                      i32.const 1
                      i32.shl
                      local.set 4
                      local.get 6
                      local.get 9
                      i32.const 4
                      i32.and
                      i32.add
                      i32.const 16
                      i32.add
                      local.tee 7
                      i32.load
                      local.tee 9
                      br_if 0 (;@9;)
                    end
                    local.get 7
                    local.get 3
                    i32.store
                    local.get 3
                    local.get 6
                    i32.store offset=24
                    local.get 3
                    local.get 3
                    i32.store offset=12
                    local.get 3
                    local.get 3
                    i32.store offset=8
                    br 3 (;@5;)
                  end
                  local.get 6
                  i32.load offset=8
                  local.tee 4
                  local.get 5
                  i32.store offset=12
                  local.get 6
                  local.get 5
                  i32.store offset=8
                  local.get 5
                  i32.const 0
                  i32.store offset=24
                  local.get 5
                  local.get 6
                  i32.store offset=12
                  local.get 5
                  local.get 4
                  i32.store offset=8
                end
                local.get 2
                i32.const 8
                i32.add
                local.set 4
                br 5 (;@1;)
              end
              local.get 6
              i32.load offset=8
              local.tee 4
              local.get 3
              i32.store offset=12
              local.get 6
              local.get 3
              i32.store offset=8
              local.get 3
              i32.const 0
              i32.store offset=24
              local.get 3
              local.get 6
              i32.store offset=12
              local.get 3
              local.get 4
              i32.store offset=8
            end
            i32.const 0
            i32.load offset=1060
            local.tee 4
            local.get 5
            i32.le_u
            br_if 0 (;@4;)
            i32.const 0
            i32.load offset=1072
            local.tee 3
            local.get 5
            i32.add
            local.tee 6
            local.get 4
            local.get 5
            i32.sub
            local.tee 4
            i32.const 1
            i32.or
            i32.store offset=4
            i32.const 0
            local.get 4
            i32.store offset=1060
            i32.const 0
            local.get 6
            i32.store offset=1072
            local.get 3
            local.get 5
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 3
            i32.const 8
            i32.add
            local.set 4
            br 3 (;@1;)
          end
          i32.const 0
          local.set 4
          i32.const 0
          i32.const 48
          i32.store offset=1544
          br 2 (;@1;)
        end
        block  ;; label = @3
          local.get 2
          i32.eqz
          br_if 0 (;@3;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 9
              local.get 9
              i32.load offset=28
              local.tee 6
              i32.const 2
              i32.shl
              i32.const 1352
              i32.add
              local.tee 4
              i32.load
              i32.ne
              br_if 0 (;@5;)
              local.get 4
              local.get 0
              i32.store
              local.get 0
              br_if 1 (;@4;)
              i32.const 0
              local.get 10
              i32.const -2
              local.get 6
              i32.rotl
              i32.and
              local.tee 10
              i32.store offset=1052
              br 2 (;@3;)
            end
            local.get 2
            i32.const 16
            i32.const 20
            local.get 2
            i32.load offset=16
            local.get 9
            i32.eq
            select
            i32.add
            local.get 0
            i32.store
            local.get 0
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 0
          local.get 2
          i32.store offset=24
          block  ;; label = @4
            local.get 9
            i32.load offset=16
            local.tee 4
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            local.get 4
            i32.store offset=16
            local.get 4
            local.get 0
            i32.store offset=24
          end
          local.get 9
          i32.const 20
          i32.add
          i32.load
          local.tee 4
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          i32.const 20
          i32.add
          local.get 4
          i32.store
          local.get 4
          local.get 0
          i32.store offset=24
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 3
            i32.const 15
            i32.gt_u
            br_if 0 (;@4;)
            local.get 9
            local.get 3
            local.get 5
            i32.add
            local.tee 4
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 9
            local.get 4
            i32.add
            local.tee 4
            local.get 4
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            br 1 (;@3;)
          end
          local.get 9
          local.get 5
          i32.add
          local.tee 0
          local.get 3
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 9
          local.get 5
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 0
          local.get 3
          i32.add
          local.get 3
          i32.store
          block  ;; label = @4
            local.get 3
            i32.const 255
            i32.gt_u
            br_if 0 (;@4;)
            local.get 3
            i32.const -8
            i32.and
            i32.const 1088
            i32.add
            local.set 4
            block  ;; label = @5
              block  ;; label = @6
                i32.const 0
                i32.load offset=1048
                local.tee 6
                i32.const 1
                local.get 3
                i32.const 3
                i32.shr_u
                i32.shl
                local.tee 3
                i32.and
                br_if 0 (;@6;)
                i32.const 0
                local.get 6
                local.get 3
                i32.or
                i32.store offset=1048
                local.get 4
                local.set 3
                br 1 (;@5;)
              end
              local.get 4
              i32.load offset=8
              local.set 3
            end
            local.get 3
            local.get 0
            i32.store offset=12
            local.get 4
            local.get 0
            i32.store offset=8
            local.get 0
            local.get 4
            i32.store offset=12
            local.get 0
            local.get 3
            i32.store offset=8
            br 1 (;@3;)
          end
          i32.const 31
          local.set 4
          block  ;; label = @4
            local.get 3
            i32.const 16777215
            i32.gt_u
            br_if 0 (;@4;)
            local.get 3
            i32.const 8
            i32.shr_u
            local.tee 4
            local.get 4
            i32.const 1048320
            i32.add
            i32.const 16
            i32.shr_u
            i32.const 8
            i32.and
            local.tee 4
            i32.shl
            local.tee 6
            local.get 6
            i32.const 520192
            i32.add
            i32.const 16
            i32.shr_u
            i32.const 4
            i32.and
            local.tee 6
            i32.shl
            local.tee 5
            local.get 5
            i32.const 245760
            i32.add
            i32.const 16
            i32.shr_u
            i32.const 2
            i32.and
            local.tee 5
            i32.shl
            i32.const 15
            i32.shr_u
            local.get 4
            local.get 6
            i32.or
            local.get 5
            i32.or
            i32.sub
            local.tee 4
            i32.const 1
            i32.shl
            local.get 3
            local.get 4
            i32.const 21
            i32.add
            i32.shr_u
            i32.const 1
            i32.and
            i32.or
            i32.const 28
            i32.add
            local.set 4
          end
          local.get 0
          local.get 4
          i32.store offset=28
          local.get 0
          i64.const 0
          i64.store offset=16 align=4
          local.get 4
          i32.const 2
          i32.shl
          i32.const 1352
          i32.add
          local.set 6
          block  ;; label = @4
            local.get 10
            i32.const 1
            local.get 4
            i32.shl
            local.tee 5
            i32.and
            br_if 0 (;@4;)
            local.get 6
            local.get 0
            i32.store
            i32.const 0
            local.get 10
            local.get 5
            i32.or
            i32.store offset=1052
            local.get 0
            local.get 6
            i32.store offset=24
            local.get 0
            local.get 0
            i32.store offset=8
            local.get 0
            local.get 0
            i32.store offset=12
            br 1 (;@3;)
          end
          local.get 3
          i32.const 0
          i32.const 25
          local.get 4
          i32.const 1
          i32.shr_u
          i32.sub
          local.get 4
          i32.const 31
          i32.eq
          select
          i32.shl
          local.set 4
          local.get 6
          i32.load
          local.set 5
          block  ;; label = @4
            loop  ;; label = @5
              local.get 5
              local.tee 6
              i32.load offset=4
              i32.const -8
              i32.and
              local.get 3
              i32.eq
              br_if 1 (;@4;)
              local.get 4
              i32.const 29
              i32.shr_u
              local.set 5
              local.get 4
              i32.const 1
              i32.shl
              local.set 4
              local.get 6
              local.get 5
              i32.const 4
              i32.and
              i32.add
              i32.const 16
              i32.add
              local.tee 7
              i32.load
              local.tee 5
              br_if 0 (;@5;)
            end
            local.get 7
            local.get 0
            i32.store
            local.get 0
            local.get 6
            i32.store offset=24
            local.get 0
            local.get 0
            i32.store offset=12
            local.get 0
            local.get 0
            i32.store offset=8
            br 1 (;@3;)
          end
          local.get 6
          i32.load offset=8
          local.tee 4
          local.get 0
          i32.store offset=12
          local.get 6
          local.get 0
          i32.store offset=8
          local.get 0
          i32.const 0
          i32.store offset=24
          local.get 0
          local.get 6
          i32.store offset=12
          local.get 0
          local.get 4
          i32.store offset=8
        end
        local.get 9
        i32.const 8
        i32.add
        local.set 4
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 11
        i32.eqz
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            local.get 0
            i32.load offset=28
            local.tee 6
            i32.const 2
            i32.shl
            i32.const 1352
            i32.add
            local.tee 4
            i32.load
            i32.ne
            br_if 0 (;@4;)
            local.get 4
            local.get 9
            i32.store
            local.get 9
            br_if 1 (;@3;)
            i32.const 0
            local.get 10
            i32.const -2
            local.get 6
            i32.rotl
            i32.and
            i32.store offset=1052
            br 2 (;@2;)
          end
          local.get 11
          i32.const 16
          i32.const 20
          local.get 11
          i32.load offset=16
          local.get 0
          i32.eq
          select
          i32.add
          local.get 9
          i32.store
          local.get 9
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 9
        local.get 11
        i32.store offset=24
        block  ;; label = @3
          local.get 0
          i32.load offset=16
          local.tee 4
          i32.eqz
          br_if 0 (;@3;)
          local.get 9
          local.get 4
          i32.store offset=16
          local.get 4
          local.get 9
          i32.store offset=24
        end
        local.get 0
        i32.const 20
        i32.add
        i32.load
        local.tee 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 9
        i32.const 20
        i32.add
        local.get 4
        i32.store
        local.get 4
        local.get 9
        i32.store offset=24
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 3
          i32.const 15
          i32.gt_u
          br_if 0 (;@3;)
          local.get 0
          local.get 3
          local.get 5
          i32.add
          local.tee 4
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 0
          local.get 4
          i32.add
          local.tee 4
          local.get 4
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          br 1 (;@2;)
        end
        local.get 0
        local.get 5
        i32.add
        local.tee 6
        local.get 3
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 0
        local.get 5
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 6
        local.get 3
        i32.add
        local.get 3
        i32.store
        block  ;; label = @3
          local.get 8
          i32.eqz
          br_if 0 (;@3;)
          local.get 8
          i32.const -8
          i32.and
          i32.const 1088
          i32.add
          local.set 5
          i32.const 0
          i32.load offset=1068
          local.set 4
          block  ;; label = @4
            block  ;; label = @5
              i32.const 1
              local.get 8
              i32.const 3
              i32.shr_u
              i32.shl
              local.tee 9
              local.get 7
              i32.and
              br_if 0 (;@5;)
              i32.const 0
              local.get 9
              local.get 7
              i32.or
              i32.store offset=1048
              local.get 5
              local.set 9
              br 1 (;@4;)
            end
            local.get 5
            i32.load offset=8
            local.set 9
          end
          local.get 9
          local.get 4
          i32.store offset=12
          local.get 5
          local.get 4
          i32.store offset=8
          local.get 4
          local.get 5
          i32.store offset=12
          local.get 4
          local.get 9
          i32.store offset=8
        end
        i32.const 0
        local.get 6
        i32.store offset=1068
        i32.const 0
        local.get 3
        i32.store offset=1056
      end
      local.get 0
      i32.const 8
      i32.add
      local.set 4
    end
    local.get 1
    i32.const 16
    i32.add
    global.set 0
    local.get 4)
  (func (;4;) (type 2) (param i32)
    local.get 0
    call 5)
  (func (;5;) (type 2) (param i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const -8
      i32.add
      local.tee 1
      local.get 0
      i32.const -4
      i32.add
      i32.load
      local.tee 2
      i32.const -8
      i32.and
      local.tee 0
      i32.add
      local.set 3
      block  ;; label = @2
        local.get 2
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 2
        i32.const 3
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 1
        local.get 1
        i32.load
        local.tee 2
        i32.sub
        local.tee 1
        i32.const 0
        i32.load offset=1064
        local.tee 4
        i32.lt_u
        br_if 1 (;@1;)
        local.get 2
        local.get 0
        i32.add
        local.set 0
        block  ;; label = @3
          local.get 1
          i32.const 0
          i32.load offset=1068
          i32.eq
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 2
            i32.const 255
            i32.gt_u
            br_if 0 (;@4;)
            local.get 1
            i32.load offset=8
            local.tee 4
            local.get 2
            i32.const 3
            i32.shr_u
            local.tee 5
            i32.const 3
            i32.shl
            i32.const 1088
            i32.add
            local.tee 6
            i32.eq
            drop
            block  ;; label = @5
              local.get 1
              i32.load offset=12
              local.tee 2
              local.get 4
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1048
              i32.const -2
              local.get 5
              i32.rotl
              i32.and
              i32.store offset=1048
              br 3 (;@2;)
            end
            local.get 2
            local.get 6
            i32.eq
            drop
            local.get 2
            local.get 4
            i32.store offset=8
            local.get 4
            local.get 2
            i32.store offset=12
            br 2 (;@2;)
          end
          local.get 1
          i32.load offset=24
          local.set 7
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.load offset=12
              local.tee 6
              local.get 1
              i32.eq
              br_if 0 (;@5;)
              local.get 1
              i32.load offset=8
              local.tee 2
              local.get 4
              i32.lt_u
              drop
              local.get 6
              local.get 2
              i32.store offset=8
              local.get 2
              local.get 6
              i32.store offset=12
              br 1 (;@4;)
            end
            block  ;; label = @5
              local.get 1
              i32.const 20
              i32.add
              local.tee 2
              i32.load
              local.tee 4
              br_if 0 (;@5;)
              local.get 1
              i32.const 16
              i32.add
              local.tee 2
              i32.load
              local.tee 4
              br_if 0 (;@5;)
              i32.const 0
              local.set 6
              br 1 (;@4;)
            end
            loop  ;; label = @5
              local.get 2
              local.set 5
              local.get 4
              local.tee 6
              i32.const 20
              i32.add
              local.tee 2
              i32.load
              local.tee 4
              br_if 0 (;@5;)
              local.get 6
              i32.const 16
              i32.add
              local.set 2
              local.get 6
              i32.load offset=16
              local.tee 4
              br_if 0 (;@5;)
            end
            local.get 5
            i32.const 0
            i32.store
          end
          local.get 7
          i32.eqz
          br_if 1 (;@2;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              local.get 1
              i32.load offset=28
              local.tee 4
              i32.const 2
              i32.shl
              i32.const 1352
              i32.add
              local.tee 2
              i32.load
              i32.ne
              br_if 0 (;@5;)
              local.get 2
              local.get 6
              i32.store
              local.get 6
              br_if 1 (;@4;)
              i32.const 0
              i32.const 0
              i32.load offset=1052
              i32.const -2
              local.get 4
              i32.rotl
              i32.and
              i32.store offset=1052
              br 3 (;@2;)
            end
            local.get 7
            i32.const 16
            i32.const 20
            local.get 7
            i32.load offset=16
            local.get 1
            i32.eq
            select
            i32.add
            local.get 6
            i32.store
            local.get 6
            i32.eqz
            br_if 2 (;@2;)
          end
          local.get 6
          local.get 7
          i32.store offset=24
          block  ;; label = @4
            local.get 1
            i32.load offset=16
            local.tee 2
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            local.get 2
            i32.store offset=16
            local.get 2
            local.get 6
            i32.store offset=24
          end
          local.get 1
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 1 (;@2;)
          local.get 6
          i32.const 20
          i32.add
          local.get 2
          i32.store
          local.get 2
          local.get 6
          i32.store offset=24
          br 1 (;@2;)
        end
        local.get 3
        i32.load offset=4
        local.tee 2
        i32.const 3
        i32.and
        i32.const 3
        i32.ne
        br_if 0 (;@2;)
        local.get 3
        local.get 2
        i32.const -2
        i32.and
        i32.store offset=4
        i32.const 0
        local.get 0
        i32.store offset=1056
        local.get 1
        local.get 0
        i32.add
        local.get 0
        i32.store
        local.get 1
        local.get 0
        i32.const 1
        i32.or
        i32.store offset=4
        return
      end
      local.get 1
      local.get 3
      i32.ge_u
      br_if 0 (;@1;)
      local.get 3
      i32.load offset=4
      local.tee 2
      i32.const 1
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.const 2
          i32.and
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 3
            i32.const 0
            i32.load offset=1072
            i32.ne
            br_if 0 (;@4;)
            i32.const 0
            local.get 1
            i32.store offset=1072
            i32.const 0
            i32.const 0
            i32.load offset=1060
            local.get 0
            i32.add
            local.tee 0
            i32.store offset=1060
            local.get 1
            local.get 0
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 1
            i32.const 0
            i32.load offset=1068
            i32.ne
            br_if 3 (;@1;)
            i32.const 0
            i32.const 0
            i32.store offset=1056
            i32.const 0
            i32.const 0
            i32.store offset=1068
            return
          end
          block  ;; label = @4
            local.get 3
            i32.const 0
            i32.load offset=1068
            i32.ne
            br_if 0 (;@4;)
            i32.const 0
            local.get 1
            i32.store offset=1068
            i32.const 0
            i32.const 0
            i32.load offset=1056
            local.get 0
            i32.add
            local.tee 0
            i32.store offset=1056
            local.get 1
            local.get 0
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 1
            local.get 0
            i32.add
            local.get 0
            i32.store
            return
          end
          local.get 2
          i32.const -8
          i32.and
          local.get 0
          i32.add
          local.set 0
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              i32.const 255
              i32.gt_u
              br_if 0 (;@5;)
              local.get 3
              i32.load offset=8
              local.tee 4
              local.get 2
              i32.const 3
              i32.shr_u
              local.tee 5
              i32.const 3
              i32.shl
              i32.const 1088
              i32.add
              local.tee 6
              i32.eq
              drop
              block  ;; label = @6
                local.get 3
                i32.load offset=12
                local.tee 2
                local.get 4
                i32.ne
                br_if 0 (;@6;)
                i32.const 0
                i32.const 0
                i32.load offset=1048
                i32.const -2
                local.get 5
                i32.rotl
                i32.and
                i32.store offset=1048
                br 2 (;@4;)
              end
              local.get 2
              local.get 6
              i32.eq
              drop
              local.get 2
              local.get 4
              i32.store offset=8
              local.get 4
              local.get 2
              i32.store offset=12
              br 1 (;@4;)
            end
            local.get 3
            i32.load offset=24
            local.set 7
            block  ;; label = @5
              block  ;; label = @6
                local.get 3
                i32.load offset=12
                local.tee 6
                local.get 3
                i32.eq
                br_if 0 (;@6;)
                local.get 3
                i32.load offset=8
                local.tee 2
                i32.const 0
                i32.load offset=1064
                i32.lt_u
                drop
                local.get 6
                local.get 2
                i32.store offset=8
                local.get 2
                local.get 6
                i32.store offset=12
                br 1 (;@5;)
              end
              block  ;; label = @6
                local.get 3
                i32.const 20
                i32.add
                local.tee 2
                i32.load
                local.tee 4
                br_if 0 (;@6;)
                local.get 3
                i32.const 16
                i32.add
                local.tee 2
                i32.load
                local.tee 4
                br_if 0 (;@6;)
                i32.const 0
                local.set 6
                br 1 (;@5;)
              end
              loop  ;; label = @6
                local.get 2
                local.set 5
                local.get 4
                local.tee 6
                i32.const 20
                i32.add
                local.tee 2
                i32.load
                local.tee 4
                br_if 0 (;@6;)
                local.get 6
                i32.const 16
                i32.add
                local.set 2
                local.get 6
                i32.load offset=16
                local.tee 4
                br_if 0 (;@6;)
              end
              local.get 5
              i32.const 0
              i32.store
            end
            local.get 7
            i32.eqz
            br_if 0 (;@4;)
            block  ;; label = @5
              block  ;; label = @6
                local.get 3
                local.get 3
                i32.load offset=28
                local.tee 4
                i32.const 2
                i32.shl
                i32.const 1352
                i32.add
                local.tee 2
                i32.load
                i32.ne
                br_if 0 (;@6;)
                local.get 2
                local.get 6
                i32.store
                local.get 6
                br_if 1 (;@5;)
                i32.const 0
                i32.const 0
                i32.load offset=1052
                i32.const -2
                local.get 4
                i32.rotl
                i32.and
                i32.store offset=1052
                br 2 (;@4;)
              end
              local.get 7
              i32.const 16
              i32.const 20
              local.get 7
              i32.load offset=16
              local.get 3
              i32.eq
              select
              i32.add
              local.get 6
              i32.store
              local.get 6
              i32.eqz
              br_if 1 (;@4;)
            end
            local.get 6
            local.get 7
            i32.store offset=24
            block  ;; label = @5
              local.get 3
              i32.load offset=16
              local.tee 2
              i32.eqz
              br_if 0 (;@5;)
              local.get 6
              local.get 2
              i32.store offset=16
              local.get 2
              local.get 6
              i32.store offset=24
            end
            local.get 3
            i32.load offset=20
            local.tee 2
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            i32.const 20
            i32.add
            local.get 2
            i32.store
            local.get 2
            local.get 6
            i32.store offset=24
          end
          local.get 1
          local.get 0
          i32.add
          local.get 0
          i32.store
          local.get 1
          local.get 0
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 1
          i32.const 0
          i32.load offset=1068
          i32.ne
          br_if 1 (;@2;)
          i32.const 0
          local.get 0
          i32.store offset=1056
          return
        end
        local.get 3
        local.get 2
        i32.const -2
        i32.and
        i32.store offset=4
        local.get 1
        local.get 0
        i32.add
        local.get 0
        i32.store
        local.get 1
        local.get 0
        i32.const 1
        i32.or
        i32.store offset=4
      end
      block  ;; label = @2
        local.get 0
        i32.const 255
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const -8
        i32.and
        i32.const 1088
        i32.add
        local.set 2
        block  ;; label = @3
          block  ;; label = @4
            i32.const 0
            i32.load offset=1048
            local.tee 4
            i32.const 1
            local.get 0
            i32.const 3
            i32.shr_u
            i32.shl
            local.tee 0
            i32.and
            br_if 0 (;@4;)
            i32.const 0
            local.get 4
            local.get 0
            i32.or
            i32.store offset=1048
            local.get 2
            local.set 0
            br 1 (;@3;)
          end
          local.get 2
          i32.load offset=8
          local.set 0
        end
        local.get 0
        local.get 1
        i32.store offset=12
        local.get 2
        local.get 1
        i32.store offset=8
        local.get 1
        local.get 2
        i32.store offset=12
        local.get 1
        local.get 0
        i32.store offset=8
        return
      end
      i32.const 31
      local.set 2
      block  ;; label = @2
        local.get 0
        i32.const 16777215
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const 8
        i32.shr_u
        local.tee 2
        local.get 2
        i32.const 1048320
        i32.add
        i32.const 16
        i32.shr_u
        i32.const 8
        i32.and
        local.tee 2
        i32.shl
        local.tee 4
        local.get 4
        i32.const 520192
        i32.add
        i32.const 16
        i32.shr_u
        i32.const 4
        i32.and
        local.tee 4
        i32.shl
        local.tee 6
        local.get 6
        i32.const 245760
        i32.add
        i32.const 16
        i32.shr_u
        i32.const 2
        i32.and
        local.tee 6
        i32.shl
        i32.const 15
        i32.shr_u
        local.get 2
        local.get 4
        i32.or
        local.get 6
        i32.or
        i32.sub
        local.tee 2
        i32.const 1
        i32.shl
        local.get 0
        local.get 2
        i32.const 21
        i32.add
        i32.shr_u
        i32.const 1
        i32.and
        i32.or
        i32.const 28
        i32.add
        local.set 2
      end
      local.get 1
      local.get 2
      i32.store offset=28
      local.get 1
      i64.const 0
      i64.store offset=16 align=4
      local.get 2
      i32.const 2
      i32.shl
      i32.const 1352
      i32.add
      local.set 4
      block  ;; label = @2
        block  ;; label = @3
          i32.const 0
          i32.load offset=1052
          local.tee 6
          i32.const 1
          local.get 2
          i32.shl
          local.tee 3
          i32.and
          br_if 0 (;@3;)
          local.get 4
          local.get 1
          i32.store
          i32.const 0
          local.get 6
          local.get 3
          i32.or
          i32.store offset=1052
          local.get 1
          local.get 4
          i32.store offset=24
          local.get 1
          local.get 1
          i32.store offset=8
          local.get 1
          local.get 1
          i32.store offset=12
          br 1 (;@2;)
        end
        local.get 0
        i32.const 0
        i32.const 25
        local.get 2
        i32.const 1
        i32.shr_u
        i32.sub
        local.get 2
        i32.const 31
        i32.eq
        select
        i32.shl
        local.set 2
        local.get 4
        i32.load
        local.set 6
        block  ;; label = @3
          loop  ;; label = @4
            local.get 6
            local.tee 4
            i32.load offset=4
            i32.const -8
            i32.and
            local.get 0
            i32.eq
            br_if 1 (;@3;)
            local.get 2
            i32.const 29
            i32.shr_u
            local.set 6
            local.get 2
            i32.const 1
            i32.shl
            local.set 2
            local.get 4
            local.get 6
            i32.const 4
            i32.and
            i32.add
            i32.const 16
            i32.add
            local.tee 3
            i32.load
            local.tee 6
            br_if 0 (;@4;)
          end
          local.get 3
          local.get 1
          i32.store
          local.get 1
          local.get 4
          i32.store offset=24
          local.get 1
          local.get 1
          i32.store offset=12
          local.get 1
          local.get 1
          i32.store offset=8
          br 1 (;@2;)
        end
        local.get 4
        i32.load offset=8
        local.tee 0
        local.get 1
        i32.store offset=12
        local.get 4
        local.get 1
        i32.store offset=8
        local.get 1
        i32.const 0
        i32.store offset=24
        local.get 1
        local.get 4
        i32.store offset=12
        local.get 1
        local.get 0
        i32.store offset=8
      end
      i32.const 0
      i32.const 0
      i32.load offset=1080
      i32.const -1
      i32.add
      local.tee 1
      i32.const -1
      local.get 1
      select
      i32.store offset=1080
    end)
  (func (;6;) (type 3) (param i32 i32) (result i32)
    (local i32 i64)
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        br_if 0 (;@2;)
        i32.const 0
        local.set 2
        br 1 (;@1;)
      end
      local.get 0
      i64.extend_i32_u
      local.get 1
      i64.extend_i32_u
      i64.mul
      local.tee 3
      i32.wrap_i64
      local.set 2
      local.get 1
      local.get 0
      i32.or
      i32.const 65536
      i32.lt_u
      br_if 0 (;@1;)
      i32.const -1
      local.get 2
      local.get 3
      i64.const 32
      i64.shr_u
      i32.wrap_i64
      i32.const 0
      i32.ne
      select
      local.set 2
    end
    block  ;; label = @1
      local.get 2
      call 3
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const -4
      i32.add
      i32.load8_u
      i32.const 3
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const 0
      local.get 2
      call 10
      drop
    end
    local.get 0)
  (func (;7;) (type 0)
    unreachable
    unreachable)
  (func (;8;) (type 1) (param i32) (result i32)
    block  ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      memory.size
      i32.const 16
      i32.shl
      return
    end
    block  ;; label = @1
      local.get 0
      i32.const 65535
      i32.and
      br_if 0 (;@1;)
      local.get 0
      i32.const -1
      i32.le_s
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 0
        i32.const 16
        i32.shr_u
        memory.grow
        local.tee 0
        i32.const -1
        i32.ne
        br_if 0 (;@2;)
        i32.const 0
        i32.const 48
        i32.store offset=1544
        i32.const -1
        return
      end
      local.get 0
      i32.const 16
      i32.shl
      return
    end
    call 7
    unreachable)
  (func (;9;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.const 32
          i32.gt_u
          br_if 0 (;@3;)
          local.get 1
          i32.const 3
          i32.and
          i32.eqz
          br_if 1 (;@2;)
          local.get 2
          i32.eqz
          br_if 1 (;@2;)
          local.get 0
          local.get 1
          i32.load8_u
          i32.store8
          local.get 2
          i32.const -1
          i32.add
          local.set 3
          local.get 0
          i32.const 1
          i32.add
          local.set 4
          local.get 1
          i32.const 1
          i32.add
          local.tee 5
          i32.const 3
          i32.and
          i32.eqz
          br_if 2 (;@1;)
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
          local.get 0
          local.get 1
          i32.load8_u offset=1
          i32.store8 offset=1
          local.get 2
          i32.const -2
          i32.add
          local.set 3
          local.get 0
          i32.const 2
          i32.add
          local.set 4
          local.get 1
          i32.const 2
          i32.add
          local.tee 5
          i32.const 3
          i32.and
          i32.eqz
          br_if 2 (;@1;)
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
          local.get 0
          local.get 1
          i32.load8_u offset=2
          i32.store8 offset=2
          local.get 2
          i32.const -3
          i32.add
          local.set 3
          local.get 0
          i32.const 3
          i32.add
          local.set 4
          local.get 1
          i32.const 3
          i32.add
          local.tee 5
          i32.const 3
          i32.and
          i32.eqz
          br_if 2 (;@1;)
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
          local.get 0
          local.get 1
          i32.load8_u offset=3
          i32.store8 offset=3
          local.get 2
          i32.const -4
          i32.add
          local.set 3
          local.get 0
          i32.const 4
          i32.add
          local.set 4
          local.get 1
          i32.const 4
          i32.add
          local.set 5
          br 2 (;@1;)
        end
        local.get 0
        local.get 1
        local.get 2
        memory.copy
        local.get 0
        return
      end
      local.get 2
      local.set 3
      local.get 0
      local.set 4
      local.get 1
      local.set 5
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 4
        i32.const 3
        i32.and
        local.tee 2
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            local.get 3
            i32.const 16
            i32.ge_u
            br_if 0 (;@4;)
            local.get 3
            local.set 2
            br 1 (;@3;)
          end
          block  ;; label = @4
            local.get 3
            i32.const -16
            i32.add
            local.tee 2
            i32.const 16
            i32.and
            br_if 0 (;@4;)
            local.get 4
            local.get 5
            i64.load align=4
            i64.store align=4
            local.get 4
            local.get 5
            i64.load offset=8 align=4
            i64.store offset=8 align=4
            local.get 4
            i32.const 16
            i32.add
            local.set 4
            local.get 5
            i32.const 16
            i32.add
            local.set 5
            local.get 2
            local.set 3
          end
          local.get 2
          i32.const 16
          i32.lt_u
          br_if 0 (;@3;)
          local.get 3
          local.set 2
          loop  ;; label = @4
            local.get 4
            local.get 5
            i64.load align=4
            i64.store align=4
            local.get 4
            local.get 5
            i64.load offset=8 align=4
            i64.store offset=8 align=4
            local.get 4
            local.get 5
            i64.load offset=16 align=4
            i64.store offset=16 align=4
            local.get 4
            local.get 5
            i64.load offset=24 align=4
            i64.store offset=24 align=4
            local.get 4
            i32.const 32
            i32.add
            local.set 4
            local.get 5
            i32.const 32
            i32.add
            local.set 5
            local.get 2
            i32.const -32
            i32.add
            local.tee 2
            i32.const 15
            i32.gt_u
            br_if 0 (;@4;)
          end
        end
        block  ;; label = @3
          local.get 2
          i32.const 8
          i32.lt_u
          br_if 0 (;@3;)
          local.get 4
          local.get 5
          i64.load align=4
          i64.store align=4
          local.get 5
          i32.const 8
          i32.add
          local.set 5
          local.get 4
          i32.const 8
          i32.add
          local.set 4
        end
        block  ;; label = @3
          local.get 2
          i32.const 4
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          local.get 5
          i32.load
          i32.store
          local.get 5
          i32.const 4
          i32.add
          local.set 5
          local.get 4
          i32.const 4
          i32.add
          local.set 4
        end
        block  ;; label = @3
          local.get 2
          i32.const 2
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          local.get 5
          i32.load16_u align=1
          i32.store16 align=1
          local.get 4
          i32.const 2
          i32.add
          local.set 4
          local.get 5
          i32.const 2
          i32.add
          local.set 5
        end
        local.get 2
        i32.const 1
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 4
        local.get 5
        i32.load8_u
        i32.store8
        local.get 0
        return
      end
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 3
                i32.const 32
                i32.lt_u
                br_if 0 (;@6;)
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 2
                    i32.const -1
                    i32.add
                    br_table 3 (;@5;) 0 (;@8;) 1 (;@7;) 7 (;@1;)
                  end
                  local.get 4
                  local.get 5
                  i32.load
                  i32.store16 align=1
                  local.get 4
                  local.get 5
                  i32.const 2
                  i32.add
                  i32.load align=2
                  i32.store offset=2
                  local.get 4
                  local.get 5
                  i32.const 6
                  i32.add
                  i64.load align=2
                  i64.store offset=6 align=4
                  local.get 4
                  i32.const 18
                  i32.add
                  local.set 2
                  local.get 5
                  i32.const 18
                  i32.add
                  local.set 1
                  i32.const 14
                  local.set 6
                  local.get 5
                  i32.const 14
                  i32.add
                  i32.load align=2
                  local.set 5
                  i32.const 14
                  local.set 3
                  br 3 (;@4;)
                end
                local.get 4
                local.get 5
                i32.load
                i32.store8
                local.get 4
                local.get 5
                i32.const 1
                i32.add
                i32.load align=1
                i32.store offset=1
                local.get 4
                local.get 5
                i32.const 5
                i32.add
                i64.load align=1
                i64.store offset=5 align=4
                local.get 4
                i32.const 17
                i32.add
                local.set 2
                local.get 5
                i32.const 17
                i32.add
                local.set 1
                i32.const 13
                local.set 6
                local.get 5
                i32.const 13
                i32.add
                i32.load align=1
                local.set 5
                i32.const 15
                local.set 3
                br 2 (;@4;)
              end
              block  ;; label = @6
                block  ;; label = @7
                  local.get 3
                  i32.const 16
                  i32.ge_u
                  br_if 0 (;@7;)
                  local.get 4
                  local.set 2
                  local.get 5
                  local.set 1
                  br 1 (;@6;)
                end
                local.get 4
                local.get 5
                i32.load8_u
                i32.store8
                local.get 4
                local.get 5
                i32.load offset=1 align=1
                i32.store offset=1 align=1
                local.get 4
                local.get 5
                i64.load offset=5 align=1
                i64.store offset=5 align=1
                local.get 4
                local.get 5
                i32.load16_u offset=13 align=1
                i32.store16 offset=13 align=1
                local.get 4
                local.get 5
                i32.load8_u offset=15
                i32.store8 offset=15
                local.get 4
                i32.const 16
                i32.add
                local.set 2
                local.get 5
                i32.const 16
                i32.add
                local.set 1
              end
              local.get 3
              i32.const 8
              i32.and
              br_if 2 (;@3;)
              br 3 (;@2;)
            end
            local.get 4
            local.get 5
            i32.load
            local.tee 2
            i32.store8
            local.get 4
            local.get 2
            i32.const 16
            i32.shr_u
            i32.store8 offset=2
            local.get 4
            local.get 2
            i32.const 8
            i32.shr_u
            i32.store8 offset=1
            local.get 4
            local.get 5
            i32.const 3
            i32.add
            i32.load align=1
            i32.store offset=3
            local.get 4
            local.get 5
            i32.const 7
            i32.add
            i64.load align=1
            i64.store offset=7 align=4
            local.get 4
            i32.const 19
            i32.add
            local.set 2
            local.get 5
            i32.const 19
            i32.add
            local.set 1
            i32.const 15
            local.set 6
            local.get 5
            i32.const 15
            i32.add
            i32.load align=1
            local.set 5
            i32.const 13
            local.set 3
          end
          local.get 4
          local.get 6
          i32.add
          local.get 5
          i32.store
        end
        local.get 2
        local.get 1
        i64.load align=1
        i64.store align=1
        local.get 2
        i32.const 8
        i32.add
        local.set 2
        local.get 1
        i32.const 8
        i32.add
        local.set 1
      end
      block  ;; label = @2
        local.get 3
        i32.const 4
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 1
        i32.load align=1
        i32.store align=1
        local.get 2
        i32.const 4
        i32.add
        local.set 2
        local.get 1
        i32.const 4
        i32.add
        local.set 1
      end
      block  ;; label = @2
        local.get 3
        i32.const 2
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 1
        i32.load16_u align=1
        i32.store16 align=1
        local.get 2
        i32.const 2
        i32.add
        local.set 2
        local.get 1
        i32.const 2
        i32.add
        local.set 1
      end
      local.get 3
      i32.const 1
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      local.get 1
      i32.load8_u
      i32.store8
    end
    local.get 0)
  (func (;10;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i64)
    block  ;; label = @1
      local.get 2
      i32.const 33
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      local.get 2
      memory.fill
      local.get 0
      return
    end
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.store8
      local.get 2
      local.get 0
      i32.add
      local.tee 3
      i32.const -1
      i32.add
      local.get 1
      i32.store8
      local.get 2
      i32.const 3
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.store8 offset=2
      local.get 0
      local.get 1
      i32.store8 offset=1
      local.get 3
      i32.const -3
      i32.add
      local.get 1
      i32.store8
      local.get 3
      i32.const -2
      i32.add
      local.get 1
      i32.store8
      local.get 2
      i32.const 7
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.store8 offset=3
      local.get 3
      i32.const -4
      i32.add
      local.get 1
      i32.store8
      local.get 2
      i32.const 9
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      i32.const 0
      local.get 0
      i32.sub
      i32.const 3
      i32.and
      local.tee 4
      i32.add
      local.tee 5
      local.get 1
      i32.const 255
      i32.and
      i32.const 16843009
      i32.mul
      local.tee 3
      i32.store
      local.get 5
      local.get 2
      local.get 4
      i32.sub
      i32.const -4
      i32.and
      local.tee 1
      i32.add
      local.tee 2
      i32.const -4
      i32.add
      local.get 3
      i32.store
      local.get 1
      i32.const 9
      i32.lt_u
      br_if 0 (;@1;)
      local.get 5
      local.get 3
      i32.store offset=8
      local.get 5
      local.get 3
      i32.store offset=4
      local.get 2
      i32.const -8
      i32.add
      local.get 3
      i32.store
      local.get 2
      i32.const -12
      i32.add
      local.get 3
      i32.store
      local.get 1
      i32.const 25
      i32.lt_u
      br_if 0 (;@1;)
      local.get 5
      local.get 3
      i32.store offset=24
      local.get 5
      local.get 3
      i32.store offset=20
      local.get 5
      local.get 3
      i32.store offset=16
      local.get 5
      local.get 3
      i32.store offset=12
      local.get 2
      i32.const -16
      i32.add
      local.get 3
      i32.store
      local.get 2
      i32.const -20
      i32.add
      local.get 3
      i32.store
      local.get 2
      i32.const -24
      i32.add
      local.get 3
      i32.store
      local.get 2
      i32.const -28
      i32.add
      local.get 3
      i32.store
      local.get 1
      local.get 5
      i32.const 4
      i32.and
      i32.const 24
      i32.or
      local.tee 2
      i32.sub
      local.tee 1
      i32.const 32
      i32.lt_u
      br_if 0 (;@1;)
      local.get 3
      i64.extend_i32_u
      i64.const 4294967297
      i64.mul
      local.set 6
      local.get 5
      local.get 2
      i32.add
      local.set 2
      loop  ;; label = @2
        local.get 2
        local.get 6
        i64.store offset=24
        local.get 2
        local.get 6
        i64.store offset=16
        local.get 2
        local.get 6
        i64.store offset=8
        local.get 2
        local.get 6
        i64.store
        local.get 2
        i32.const 32
        i32.add
        local.set 2
        local.get 1
        i32.const -32
        i32.add
        local.tee 1
        i32.const 31
        i32.gt_u
        br_if 0 (;@2;)
      end
    end
    local.get 0)
  (func (;11;) (type 1) (param i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    local.get 0
    i32.store offset=8
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.load offset=8
        i32.const 0
        i32.gt_u
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        local.get 1
        i32.load offset=8
        call 2
        i32.store offset=12
        br 1 (;@1;)
      end
      local.get 1
      i32.const 1024
      i32.store offset=12
    end
    local.get 1
    i32.load offset=12
    local.set 0
    local.get 1
    i32.const 16
    i32.add
    global.set 0
    local.get 0)
  (func (;12;) (type 2) (param i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    local.get 0
    i32.store offset=12
    block  ;; label = @1
      local.get 1
      i32.load offset=12
      i32.const 1024
      i32.ne
      i32.const 1
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.load offset=12
      call 4
    end
    local.get 1
    i32.const 16
    i32.add
    global.set 0)
  (func (;13;) (type 2) (param i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    local.get 0
    i32.store offset=12
    local.get 1
    i32.load offset=12
    call 12
    local.get 1
    i32.const 16
    i32.add
    global.set 0)
  (func (;14;) (type 4) (param i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    local.get 0
    i32.store offset=40
    local.get 3
    local.get 1
    i32.store offset=36
    local.get 3
    local.get 2
    i32.store offset=32
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        i32.load offset=32
        i32.load
        call 15
        i32.const 255
        i32.and
        i32.const 0
        i32.const 255
        i32.and
        i32.ne
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.const 0
        i32.store offset=44
        br 1 (;@1;)
      end
      call 16
      local.set 0
      local.get 3
      i32.const 0
      i32.const 1
      i32.and
      i32.store8 offset=19
      i32.const 0
      local.set 1
      block  ;; label = @2
        local.get 0
        i32.const 0
        i32.eq
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 3
        local.get 0
        i32.store offset=20
        local.get 3
        i32.const 1
        i32.const 1
        i32.and
        i32.store8 offset=19
        local.get 0
        local.get 3
        i32.load offset=40
        local.get 3
        i32.load offset=36
        local.get 3
        i32.load offset=32
        call 17
        local.set 1
      end
      local.get 3
      i32.const 24
      i32.add
      local.get 1
      local.get 3
      i32.load offset=32
      call 18
      local.set 0
      local.get 3
      i32.load offset=32
      i32.load
      call 15
      local.set 1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.const 255
          i32.and
          i32.const 0
          i32.const 255
          i32.and
          i32.ne
          i32.const 1
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          i32.const 0
          i32.store offset=44
          local.get 3
          i32.const 1
          i32.store offset=4
          br 1 (;@2;)
        end
        local.get 0
        call 19
        local.set 1
        local.get 3
        local.get 1
        i32.store offset=44
        local.get 3
        i32.const 1
        i32.store offset=4
      end
      local.get 0
      call 20
      drop
    end
    local.get 3
    i32.load offset=44
    local.set 0
    local.get 3
    i32.const 48
    i32.add
    global.set 0
    local.get 0)
  (func (;15;) (type 1) (param i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    local.get 0
    i32.store offset=12
    local.get 1
    i32.load offset=12
    i32.const 0
    i32.gt_s
    i32.const 1
    i32.and
    i32.const 24
    i32.shl
    i32.const 24
    i32.shr_s)
  (func (;16;) (type 5) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 0
    global.set 0
    local.get 0
    i32.const 69684
    i32.store offset=12
    local.get 0
    i32.load offset=12
    call 11
    local.set 1
    local.get 0
    i32.const 16
    i32.add
    global.set 0
    local.get 1)
  (func (;17;) (type 6) (param i32 i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 4
    global.set 0
    local.get 4
    local.get 0
    i32.store offset=24
    local.get 4
    local.get 1
    i32.store offset=20
    local.get 4
    local.get 2
    i32.store offset=16
    local.get 4
    local.get 3
    i32.store offset=12
    local.get 4
    local.get 4
    i32.load offset=24
    local.tee 0
    i32.store offset=28
    local.get 0
    i32.const 0
    i32.store
    local.get 0
    i32.const 0
    i32.store offset=4
    local.get 0
    i32.const -1
    i32.store offset=8
    local.get 0
    i32.const 0
    i32.store offset=12
    local.get 0
    i32.const 0
    i32.store offset=16
    local.get 0
    i32.const 0
    i32.store offset=20
    local.get 0
    i32.const -1
    i32.store offset=24
    local.get 0
    local.get 4
    i32.load offset=20
    i32.store offset=28
    local.get 0
    local.get 4
    i32.load offset=20
    i32.store offset=32
    local.get 0
    local.get 4
    i32.load offset=16
    i32.store offset=36
    local.get 0
    i32.const 0
    i32.store offset=40
    local.get 0
    local.get 0
    i32.load offset=32
    i32.store offset=44
    local.get 0
    i32.const 0
    i32.store offset=48
    block  ;; label = @1
      block  ;; label = @2
        local.get 4
        i32.load offset=12
        i32.load
        call 15
        i32.const 255
        i32.and
        i32.const 0
        i32.const 255
        i32.and
        i32.ne
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        br 1 (;@1;)
      end
      local.get 0
      i32.const 16384
      call 11
      i32.store
      local.get 0
      i32.const 65536
      call 11
      i32.store offset=12
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.load
          i32.const 0
          i32.eq
          i32.const 1
          i32.and
          br_if 0 (;@3;)
          local.get 0
          i32.load offset=12
          i32.const 0
          i32.eq
          i32.const 1
          i32.and
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 4
        i32.load offset=12
        i32.const 7
        i32.store
        br 1 (;@1;)
      end
      local.get 0
      i32.const 4096
      i32.store offset=4
      local.get 0
      i32.const 16384
      i32.store offset=16
    end
    local.get 4
    i32.load offset=28
    local.set 0
    local.get 4
    i32.const 32
    i32.add
    global.set 0
    local.get 0)
  (func (;18;) (type 4) (param i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    local.get 0
    i32.store offset=8
    local.get 3
    local.get 1
    i32.store offset=4
    local.get 3
    local.get 2
    i32.store
    local.get 3
    local.get 3
    i32.load offset=8
    local.tee 0
    i32.store offset=12
    local.get 0
    local.get 3
    i32.load offset=4
    call 21
    drop
    block  ;; label = @1
      local.get 3
      i32.load offset=4
      i32.const 0
      i32.eq
      i32.const 1
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      i32.load
      i32.load
      call 22
      i32.const 255
      i32.and
      i32.const 0
      i32.const 255
      i32.and
      i32.ne
      i32.const 1
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      i32.load
      i32.const 7
      i32.store
    end
    local.get 3
    i32.load offset=12
    local.set 0
    local.get 3
    i32.const 16
    i32.add
    global.set 0
    local.get 0)
  (func (;19;) (type 1) (param i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    local.get 0
    i32.store offset=12
    local.get 1
    local.get 1
    i32.load offset=12
    local.tee 0
    i32.load
    i32.store offset=8
    local.get 0
    i32.const 0
    i32.store
    local.get 1
    i32.load offset=8)
  (func (;20;) (type 1) (param i32) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    local.get 0
    i32.store offset=8
    local.get 1
    local.get 1
    i32.load offset=8
    local.tee 0
    i32.store offset=12
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 2
      i32.const 0
      i32.eq
      i32.const 1
      i32.and
      br_if 0 (;@1;)
      local.get 2
      call 23
      call 24
    end
    local.get 0
    call 25
    drop
    local.get 1
    i32.load offset=12
    local.set 0
    local.get 1
    i32.const 16
    i32.add
    global.set 0
    local.get 0)
  (func (;21;) (type 3) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    local.get 0
    i32.store offset=12
    local.get 2
    local.get 1
    i32.store offset=8
    local.get 2
    i32.load offset=12
    local.tee 0
    local.get 2
    i32.load offset=8
    i32.store
    local.get 0)
  (func (;22;) (type 1) (param i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    local.get 0
    i32.store offset=12
    local.get 1
    i32.load offset=12
    i32.const 0
    i32.le_s
    i32.const 1
    i32.and
    i32.const 24
    i32.shl
    i32.const 24
    i32.shr_s)
  (func (;23;) (type 1) (param i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    local.get 0
    i32.store offset=12
    local.get 1
    i32.load offset=12
    local.tee 0
    i32.load
    call 12
    local.get 0
    i32.load offset=12
    call 12
    local.get 0
    i32.load offset=48
    call 12
    local.get 1
    i32.const 16
    i32.add
    global.set 0
    local.get 0)
  (func (;24;) (type 2) (param i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    local.get 0
    i32.store offset=12
    block  ;; label = @1
      local.get 1
      i32.load offset=12
      i32.const 0
      i32.ne
      i32.const 1
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.load offset=12
      call 12
    end
    local.get 1
    i32.const 16
    i32.add
    global.set 0)
  (func (;25;) (type 1) (param i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    local.get 0
    i32.store offset=12
    local.get 1
    i32.load offset=12)
  (func (;26;) (type 2) (param i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    local.get 0
    i32.store offset=12
    block  ;; label = @1
      local.get 1
      i32.load offset=12
      local.tee 0
      i32.const 0
      i32.eq
      i32.const 1
      i32.and
      br_if 0 (;@1;)
      local.get 0
      call 23
      call 24
    end
    local.get 1
    i32.const 16
    i32.add
    global.set 0)
  (func (;27;) (type 7) (param i32 i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 4
    global.set 0
    local.get 4
    local.get 0
    i32.store offset=12
    local.get 4
    local.get 1
    i32.store offset=8
    local.get 4
    local.get 2
    i32.store offset=4
    local.get 4
    local.get 3
    i32.store
    block  ;; label = @1
      block  ;; label = @2
        local.get 4
        i32.load
        i32.load
        call 15
        i32.const 255
        i32.and
        i32.const 0
        i32.const 255
        i32.and
        i32.ne
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        br 1 (;@1;)
      end
      local.get 4
      i32.load offset=12
      local.get 4
      i32.load offset=8
      local.get 4
      i32.load offset=4
      local.get 4
      i32.load
      call 28
    end
    local.get 4
    i32.const 16
    i32.add
    global.set 0)
  (func (;28;) (type 7) (param i32 i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 4
    global.set 0
    local.get 4
    local.get 0
    i32.store offset=28
    local.get 4
    local.get 1
    i32.store offset=24
    local.get 4
    local.get 2
    i32.store offset=20
    local.get 4
    local.get 3
    i32.store offset=16
    local.get 4
    i32.load offset=28
    local.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 4
        i32.load offset=16
        i32.load
        call 15
        i32.const 255
        i32.and
        i32.const 0
        i32.const 255
        i32.and
        i32.ne
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 4
        i32.load offset=24
        i32.const 1114111
        i32.gt_u
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        i32.load offset=16
        i32.const 1
        i32.store
        br 1 (;@1;)
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          local.get 4
          i32.load offset=24
          call 29
          i32.const 1
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          local.get 0
          local.get 4
          i32.load offset=24
          i32.const 4
          i32.shr_s
          call 30
          local.tee 1
          i32.store offset=12
          local.get 1
          i32.const 0
          i32.lt_s
          i32.const 1
          i32.and
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 4
        i32.load offset=16
        i32.const 7
        i32.store
        br 1 (;@1;)
      end
      local.get 0
      i32.load offset=12
      local.get 4
      i32.load offset=12
      local.get 4
      i32.load offset=24
      i32.const 15
      i32.and
      i32.add
      i32.const 2
      i32.shl
      i32.add
      local.get 4
      i32.load offset=20
      i32.store
    end
    local.get 4
    i32.const 32
    i32.add
    global.set 0)
  (func (;29;) (type 3) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 0
    i32.store offset=24
    local.get 2
    local.get 1
    i32.store offset=20
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.load offset=20
        local.get 2
        i32.load offset=24
        local.tee 0
        i32.load offset=40
        i32.ge_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 2
        i32.load offset=20
        i32.const 512
        i32.add
        i32.const -512
        i32.and
        i32.store offset=20
        local.get 2
        local.get 0
        i32.load offset=40
        i32.const 4
        i32.shr_s
        i32.store offset=16
        local.get 2
        local.get 2
        i32.load offset=20
        i32.const 4
        i32.shr_s
        i32.store offset=12
        block  ;; label = @3
          local.get 2
          i32.load offset=12
          local.get 0
          i32.load offset=4
          i32.gt_s
          i32.const 1
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          i32.const 278528
          call 11
          i32.store offset=8
          block  ;; label = @4
            local.get 2
            i32.load offset=8
            i32.const 0
            i32.eq
            i32.const 1
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.const 0
            i32.const 1
            i32.and
            i32.store8 offset=31
            br 3 (;@1;)
          end
          local.get 2
          i32.load offset=8
          local.get 0
          i32.load
          local.get 2
          i32.load offset=16
          i32.const 2
          i32.shl
          call 9
          drop
          local.get 0
          i32.load
          call 12
          local.get 0
          local.get 2
          i32.load offset=8
          i32.store
          local.get 0
          i32.const 69632
          i32.store offset=4
        end
        loop  ;; label = @3
          local.get 0
          i32.const 52
          i32.add
          local.get 2
          i32.load offset=16
          i32.add
          i32.const 0
          i32.store8
          local.get 0
          i32.load
          local.get 2
          i32.load offset=16
          i32.const 2
          i32.shl
          i32.add
          local.get 0
          i32.load offset=32
          i32.store
          local.get 2
          local.get 2
          i32.load offset=16
          i32.const 1
          i32.add
          local.tee 1
          i32.store offset=16
          local.get 1
          local.get 2
          i32.load offset=12
          i32.lt_s
          i32.const 1
          i32.and
          br_if 0 (;@3;)
        end
        local.get 0
        local.get 2
        i32.load offset=20
        i32.store offset=40
      end
      local.get 2
      i32.const 1
      i32.const 1
      i32.and
      i32.store8 offset=31
    end
    local.get 2
    i32.load8_u offset=31
    local.set 0
    local.get 2
    i32.const 32
    i32.add
    global.set 0
    local.get 0
    i32.const 1
    i32.and)
  (func (;30;) (type 3) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 0
    i32.store offset=24
    local.get 2
    local.get 1
    i32.store offset=20
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.load offset=24
        local.tee 0
        i32.const 52
        i32.add
        local.get 2
        i32.load offset=20
        i32.add
        i32.load8_u
        i32.const 255
        i32.and
        i32.const 1
        i32.eq
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 0
        i32.load
        local.get 2
        i32.load offset=20
        i32.const 2
        i32.shl
        i32.add
        i32.load
        i32.store offset=28
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 2
        i32.load offset=20
        i32.const 4096
        i32.lt_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 0
        i32.const 64
        call 31
        i32.store offset=16
        block  ;; label = @3
          local.get 2
          i32.load offset=16
          i32.const 0
          i32.lt_s
          i32.const 1
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          local.get 2
          i32.load offset=16
          i32.store offset=28
          br 2 (;@1;)
        end
        local.get 2
        local.get 2
        i32.load offset=20
        i32.const -4
        i32.and
        i32.store offset=12
        local.get 2
        local.get 2
        i32.load offset=12
        i32.const 4
        i32.add
        i32.store offset=8
        loop  ;; label = @3
          local.get 0
          i32.load offset=12
          local.get 2
          i32.load offset=16
          i32.const 2
          i32.shl
          i32.add
          local.get 0
          i32.load
          local.get 2
          i32.load offset=12
          i32.const 2
          i32.shl
          i32.add
          i32.load
          call 32
          local.get 0
          i32.const 52
          i32.add
          local.get 2
          i32.load offset=12
          i32.add
          i32.const 1
          i32.store8
          local.get 2
          i32.load offset=16
          local.set 1
          local.get 0
          i32.load
          local.set 3
          local.get 2
          local.get 2
          i32.load offset=12
          local.tee 4
          i32.const 1
          i32.add
          i32.store offset=12
          local.get 3
          local.get 4
          i32.const 2
          i32.shl
          i32.add
          local.get 1
          i32.store
          local.get 2
          local.get 2
          i32.load offset=16
          i32.const 16
          i32.add
          i32.store offset=16
          local.get 2
          i32.load offset=12
          local.get 2
          i32.load offset=8
          i32.lt_s
          i32.const 1
          i32.and
          br_if 0 (;@3;)
        end
        local.get 2
        local.get 0
        i32.load
        local.get 2
        i32.load offset=20
        i32.const 2
        i32.shl
        i32.add
        i32.load
        i32.store offset=28
        br 1 (;@1;)
      end
      local.get 2
      local.get 0
      i32.const 16
      call 31
      i32.store offset=4
      block  ;; label = @2
        local.get 2
        i32.load offset=4
        i32.const 0
        i32.lt_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 2
        i32.load offset=4
        i32.store offset=28
        br 1 (;@1;)
      end
      local.get 0
      i32.load offset=12
      local.get 2
      i32.load offset=4
      i32.const 2
      i32.shl
      i32.add
      local.get 0
      i32.load
      local.get 2
      i32.load offset=20
      i32.const 2
      i32.shl
      i32.add
      i32.load
      call 32
      local.get 0
      i32.const 52
      i32.add
      local.get 2
      i32.load offset=20
      i32.add
      i32.const 1
      i32.store8
      local.get 0
      i32.load
      local.get 2
      i32.load offset=20
      i32.const 2
      i32.shl
      i32.add
      local.get 2
      i32.load offset=4
      i32.store
      local.get 2
      local.get 2
      i32.load offset=4
      i32.store offset=28
    end
    local.get 2
    i32.load offset=28
    local.set 0
    local.get 2
    i32.const 32
    i32.add
    global.set 0
    local.get 0)
  (func (;31;) (type 3) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 0
    i32.store offset=24
    local.get 2
    local.get 1
    i32.store offset=20
    local.get 2
    local.get 2
    i32.load offset=24
    local.tee 0
    i32.load offset=20
    i32.store offset=16
    local.get 2
    local.get 2
    i32.load offset=16
    local.get 2
    i32.load offset=20
    i32.add
    i32.store offset=12
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.load offset=12
        local.get 0
        i32.load offset=16
        i32.gt_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            i32.load offset=16
            i32.const 131072
            i32.lt_s
            i32.const 1
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.const 131072
            i32.store offset=8
            br 1 (;@3;)
          end
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.load offset=16
              i32.const 1114112
              i32.lt_s
              i32.const 1
              i32.and
              i32.eqz
              br_if 0 (;@5;)
              local.get 2
              i32.const 1114112
              i32.store offset=8
              br 1 (;@4;)
            end
            local.get 2
            i32.const -1
            i32.store offset=28
            br 3 (;@1;)
          end
        end
        local.get 2
        local.get 2
        i32.load offset=8
        i32.const 2
        i32.shl
        call 11
        i32.store offset=4
        block  ;; label = @3
          local.get 2
          i32.load offset=4
          i32.const 0
          i32.eq
          i32.const 1
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          i32.const -1
          i32.store offset=28
          br 2 (;@1;)
        end
        local.get 2
        i32.load offset=4
        local.get 0
        i32.load offset=12
        local.get 0
        i32.load offset=20
        i32.const 2
        i32.shl
        call 9
        drop
        local.get 0
        i32.load offset=12
        call 12
        local.get 0
        local.get 2
        i32.load offset=4
        i32.store offset=12
        local.get 0
        local.get 2
        i32.load offset=8
        i32.store offset=16
      end
      local.get 0
      local.get 2
      i32.load offset=12
      i32.store offset=20
      local.get 2
      local.get 2
      i32.load offset=16
      i32.store offset=28
    end
    local.get 2
    i32.load offset=28
    local.set 0
    local.get 2
    i32.const 32
    i32.add
    global.set 0
    local.get 0)
  (func (;32;) (type 8) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    local.get 0
    i32.store offset=12
    local.get 2
    local.get 1
    i32.store offset=8
    local.get 2
    local.get 2
    i32.load offset=12
    i32.const 64
    i32.add
    i32.store offset=4
    block  ;; label = @1
      loop  ;; label = @2
        local.get 2
        i32.load offset=12
        local.get 2
        i32.load offset=4
        i32.lt_u
        i32.const 1
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 2
        i32.load offset=8
        local.set 0
        local.get 2
        local.get 2
        i32.load offset=12
        local.tee 1
        i32.const 4
        i32.add
        i32.store offset=12
        local.get 1
        local.get 0
        i32.store
        br 0 (;@2;)
      end
    end)
  (func (;33;) (type 6) (param i32 i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 4
    global.set 0
    local.get 4
    local.get 0
    i32.store offset=24
    local.get 4
    local.get 1
    i32.store offset=20
    local.get 4
    local.get 2
    i32.store offset=16
    local.get 4
    local.get 3
    i32.store offset=12
    block  ;; label = @1
      block  ;; label = @2
        local.get 4
        i32.load offset=12
        i32.load
        call 15
        i32.const 255
        i32.and
        i32.const 0
        i32.const 255
        i32.and
        i32.ne
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        i32.const 0
        i32.store offset=28
        br 1 (;@1;)
      end
      local.get 4
      local.get 4
      i32.load offset=24
      local.get 4
      i32.load offset=20
      local.get 4
      i32.load offset=16
      local.get 4
      i32.load offset=12
      call 34
      i32.store offset=28
    end
    local.get 4
    i32.load offset=28
    local.set 0
    local.get 4
    i32.const 32
    i32.add
    global.set 0
    local.get 0)
  (func (;34;) (type 6) (param i32 i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 80
    i32.sub
    local.tee 4
    global.set 0
    local.get 4
    local.get 0
    i32.store offset=72
    local.get 4
    local.get 1
    i32.store offset=68
    local.get 4
    local.get 2
    i32.store offset=64
    local.get 4
    local.get 3
    i32.store offset=60
    local.get 4
    i32.load offset=72
    local.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 4
        i32.load offset=60
        i32.load
        call 15
        i32.const 255
        i32.and
        i32.const 0
        i32.const 255
        i32.and
        i32.ne
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        i32.const 0
        i32.store offset=76
        br 1 (;@1;)
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 4
          i32.load offset=68
          i32.const 0
          i32.lt_s
          i32.const 1
          i32.and
          br_if 0 (;@3;)
          i32.const 1
          local.get 4
          i32.load offset=68
          i32.lt_s
          i32.const 1
          i32.and
          br_if 0 (;@3;)
          local.get 4
          i32.load offset=64
          i32.const 0
          i32.lt_s
          i32.const 1
          i32.and
          br_if 0 (;@3;)
          i32.const 2
          local.get 4
          i32.load offset=64
          i32.lt_s
          i32.const 1
          i32.and
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 4
        i32.load offset=60
        i32.const 1
        i32.store
        local.get 4
        i32.const 0
        i32.store offset=76
        br 1 (;@1;)
      end
      local.get 4
      i32.load offset=64
      local.tee 1
      i32.const 2
      i32.gt_u
      drop
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 1
                br_table 1 (;@5;) 0 (;@6;) 2 (;@4;) 3 (;@3;)
              end
              br 3 (;@2;)
            end
            local.get 0
            i32.const 65535
            call 35
            br 2 (;@2;)
          end
          local.get 0
          i32.const 255
          call 35
          br 1 (;@2;)
        end
      end
      local.get 4
      i32.const 4096
      i32.const 65536
      local.get 4
      i32.load offset=68
      select
      i32.store offset=56
      local.get 4
      local.get 0
      local.get 4
      i32.load offset=56
      i32.const 4
      i32.shr_s
      local.get 4
      i32.load offset=60
      call 36
      i32.store offset=52
      block  ;; label = @2
        local.get 4
        i32.load offset=60
        i32.load
        call 15
        i32.const 255
        i32.and
        i32.const 0
        i32.const 255
        i32.and
        i32.ne
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        call 37
        local.get 4
        i32.const 0
        i32.store offset=76
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 4
        i32.load offset=64
        i32.const 1
        i32.eq
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        i32.load offset=52
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.load offset=48
        local.set 1
        local.get 4
        local.get 4
        i32.load offset=52
        local.tee 2
        i32.const 1
        i32.add
        i32.store offset=52
        local.get 1
        local.get 2
        i32.const 1
        i32.shl
        i32.add
        i32.const 65518
        i32.store16
      end
      local.get 4
      local.get 4
      i32.load offset=52
      i32.const 1
      i32.shl
      i32.store offset=48
      block  ;; label = @2
        block  ;; label = @3
          local.get 4
          i32.load offset=64
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 4
            i32.load offset=52
            local.get 0
            i32.load offset=20
            i32.xor
            i32.const 1
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            i32.load offset=36
            local.set 1
            local.get 0
            i32.load offset=12
            local.set 2
            local.get 0
            local.get 0
            i32.load offset=20
            local.tee 3
            i32.const 1
            i32.add
            i32.store offset=20
            local.get 2
            local.get 3
            i32.const 2
            i32.shl
            i32.add
            local.get 1
            i32.store
          end
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.load offset=12
              local.get 0
              i32.load offset=20
              i32.const 1
              i32.sub
              i32.const 2
              i32.shl
              i32.add
              i32.load
              local.get 0
              i32.load offset=36
              i32.ne
              i32.const 1
              i32.and
              br_if 0 (;@5;)
              local.get 0
              i32.load offset=12
              local.get 0
              i32.load offset=20
              i32.const 2
              i32.sub
              i32.const 2
              i32.shl
              i32.add
              i32.load
              local.get 0
              i32.load offset=44
              i32.ne
              i32.const 1
              i32.and
              i32.eqz
              br_if 1 (;@4;)
            end
            local.get 0
            i32.load offset=44
            local.set 1
            local.get 0
            i32.load offset=12
            local.set 2
            local.get 0
            local.get 0
            i32.load offset=20
            local.tee 3
            i32.const 1
            i32.add
            i32.store offset=20
            local.get 2
            local.get 3
            i32.const 2
            i32.shl
            i32.add
            local.get 1
            i32.store
            local.get 0
            i32.load offset=36
            local.set 1
            local.get 0
            i32.load offset=12
            local.set 2
            local.get 0
            local.get 0
            i32.load offset=20
            local.tee 3
            i32.const 1
            i32.add
            i32.store offset=20
            local.get 2
            local.get 3
            i32.const 2
            i32.shl
            i32.add
            local.get 1
            i32.store
          end
          local.get 4
          local.get 4
          i32.load offset=48
          local.get 0
          i32.load offset=20
          i32.const 1
          i32.shl
          i32.add
          i32.store offset=48
          br 1 (;@2;)
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 4
            i32.load offset=64
            i32.const 1
            i32.eq
            i32.const 1
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.load offset=12
                local.get 0
                i32.load offset=20
                i32.const 1
                i32.sub
                i32.const 2
                i32.shl
                i32.add
                i32.load
                local.get 0
                i32.load offset=36
                i32.ne
                i32.const 1
                i32.and
                br_if 0 (;@6;)
                local.get 0
                i32.load offset=12
                local.get 0
                i32.load offset=20
                i32.const 2
                i32.sub
                i32.const 2
                i32.shl
                i32.add
                i32.load
                local.get 0
                i32.load offset=44
                i32.ne
                i32.const 1
                i32.and
                i32.eqz
                br_if 1 (;@5;)
              end
              block  ;; label = @6
                local.get 0
                i32.load offset=12
                local.get 0
                i32.load offset=20
                i32.const 1
                i32.sub
                i32.const 2
                i32.shl
                i32.add
                i32.load
                local.get 0
                i32.load offset=44
                i32.ne
                i32.const 1
                i32.and
                i32.eqz
                br_if 0 (;@6;)
                local.get 0
                i32.load offset=44
                local.set 1
                local.get 0
                i32.load offset=12
                local.set 2
                local.get 0
                local.get 0
                i32.load offset=20
                local.tee 3
                i32.const 1
                i32.add
                i32.store offset=20
                local.get 2
                local.get 3
                i32.const 2
                i32.shl
                i32.add
                local.get 1
                i32.store
              end
              local.get 0
              i32.load offset=36
              local.set 1
              local.get 0
              i32.load offset=12
              local.set 2
              local.get 0
              local.get 0
              i32.load offset=20
              local.tee 3
              i32.const 1
              i32.add
              i32.store offset=20
              local.get 2
              local.get 3
              i32.const 2
              i32.shl
              i32.add
              local.get 1
              i32.store
            end
            local.get 4
            local.get 4
            i32.load offset=48
            local.get 0
            i32.load offset=20
            i32.const 2
            i32.shl
            i32.add
            i32.store offset=48
            br 1 (;@3;)
          end
          local.get 4
          local.get 4
          i32.load offset=48
          local.get 0
          i32.load offset=20
          i32.add
          i32.const 3
          i32.and
          i32.store offset=44
          block  ;; label = @4
            block  ;; label = @5
              local.get 4
              i32.load offset=44
              br_if 0 (;@5;)
              local.get 0
              i32.load offset=12
              local.get 0
              i32.load offset=20
              i32.const 1
              i32.sub
              i32.const 2
              i32.shl
              i32.add
              i32.load
              local.get 0
              i32.load offset=36
              i32.eq
              i32.const 1
              i32.and
              i32.eqz
              br_if 0 (;@5;)
              local.get 0
              i32.load offset=12
              local.get 0
              i32.load offset=20
              i32.const 2
              i32.sub
              i32.const 2
              i32.shl
              i32.add
              i32.load
              local.get 0
              i32.load offset=44
              i32.eq
              i32.const 1
              i32.and
              i32.eqz
              br_if 0 (;@5;)
              br 1 (;@4;)
            end
            block  ;; label = @5
              block  ;; label = @6
                local.get 4
                i32.load offset=44
                i32.const 3
                i32.eq
                i32.const 1
                i32.and
                i32.eqz
                br_if 0 (;@6;)
                local.get 0
                i32.load offset=12
                local.get 0
                i32.load offset=20
                i32.const 1
                i32.sub
                i32.const 2
                i32.shl
                i32.add
                i32.load
                local.get 0
                i32.load offset=44
                i32.eq
                i32.const 1
                i32.and
                i32.eqz
                br_if 0 (;@6;)
                local.get 0
                i32.load offset=36
                local.set 1
                local.get 0
                i32.load offset=12
                local.set 2
                local.get 0
                local.get 0
                i32.load offset=20
                local.tee 3
                i32.const 1
                i32.add
                i32.store offset=20
                local.get 2
                local.get 3
                i32.const 2
                i32.shl
                i32.add
                local.get 1
                i32.store
                br 1 (;@5;)
              end
              block  ;; label = @6
                loop  ;; label = @7
                  local.get 4
                  i32.load offset=44
                  i32.const 2
                  i32.ne
                  i32.const 1
                  i32.and
                  i32.eqz
                  br_if 1 (;@6;)
                  local.get 0
                  i32.load offset=44
                  local.set 1
                  local.get 0
                  i32.load offset=12
                  local.set 2
                  local.get 0
                  local.get 0
                  i32.load offset=20
                  local.tee 3
                  i32.const 1
                  i32.add
                  i32.store offset=20
                  local.get 2
                  local.get 3
                  i32.const 2
                  i32.shl
                  i32.add
                  local.get 1
                  i32.store
                  local.get 4
                  local.get 4
                  i32.load offset=44
                  i32.const 1
                  i32.add
                  i32.const 3
                  i32.and
                  i32.store offset=44
                  br 0 (;@7;)
                end
              end
              local.get 0
              i32.load offset=44
              local.set 1
              local.get 0
              i32.load offset=12
              local.set 2
              local.get 0
              local.get 0
              i32.load offset=20
              local.tee 3
              i32.const 1
              i32.add
              i32.store offset=20
              local.get 2
              local.get 3
              i32.const 2
              i32.shl
              i32.add
              local.get 1
              i32.store
              local.get 0
              i32.load offset=36
              local.set 1
              local.get 0
              i32.load offset=12
              local.set 2
              local.get 0
              local.get 0
              i32.load offset=20
              local.tee 3
              i32.const 1
              i32.add
              i32.store offset=20
              local.get 2
              local.get 3
              i32.const 2
              i32.shl
              i32.add
              local.get 1
              i32.store
            end
          end
          local.get 4
          local.get 4
          i32.load offset=48
          local.get 0
          i32.load offset=20
          i32.add
          i32.store offset=48
        end
      end
      local.get 4
      local.get 4
      i32.load offset=48
      i32.const 40
      i32.add
      i32.store offset=48
      local.get 4
      local.get 4
      i32.load offset=48
      call 11
      i32.store offset=40
      block  ;; label = @2
        local.get 4
        i32.load offset=40
        i32.const 0
        i32.eq
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        i32.load offset=60
        i32.const 7
        i32.store
        local.get 0
        call 37
        local.get 4
        i32.const 0
        i32.store offset=76
        br 1 (;@1;)
      end
      local.get 4
      local.get 4
      i32.load offset=40
      i32.store offset=36
      local.get 4
      i32.load offset=36
      local.tee 1
      i64.const 0
      i64.store align=4
      local.get 1
      i32.const 32
      i32.add
      i64.const 0
      i64.store align=4
      local.get 1
      i32.const 24
      i32.add
      i64.const 0
      i64.store align=4
      local.get 1
      i32.const 16
      i32.add
      i64.const 0
      i64.store align=4
      local.get 1
      i32.const 8
      i32.add
      i64.const 0
      i64.store align=4
      local.get 4
      i32.load offset=36
      local.get 4
      i32.load offset=52
      i32.store offset=8
      local.get 4
      i32.load offset=36
      local.get 0
      i32.load offset=20
      i32.store offset=12
      local.get 4
      i32.load offset=36
      local.get 0
      i32.load offset=40
      i32.store offset=16
      local.get 4
      i32.load offset=36
      local.get 0
      i32.load offset=40
      i32.const 4095
      i32.add
      i32.const 12
      i32.shr_s
      i32.store16 offset=20
      local.get 4
      i32.load offset=36
      local.get 4
      i32.load offset=68
      i32.store8 offset=22
      local.get 4
      i32.load offset=36
      local.get 4
      i32.load offset=64
      i32.store8 offset=23
      local.get 4
      i32.load offset=36
      local.get 0
      i32.load offset=8
      i32.store16 offset=30
      local.get 4
      i32.load offset=36
      local.get 0
      i32.load offset=24
      i32.store offset=32
      local.get 4
      i32.load offset=36
      local.get 0
      i32.load offset=32
      i32.store offset=36
      local.get 4
      local.get 4
      i32.load offset=40
      i32.const 40
      i32.add
      i32.store offset=40
      local.get 4
      local.get 4
      i32.load offset=40
      i32.store offset=32
      local.get 4
      i32.load offset=36
      local.get 4
      i32.load offset=32
      i32.store
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.load offset=40
          local.get 4
          i32.load offset=56
          i32.le_s
          i32.const 1
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          i32.const 0
          i32.store offset=28
          local.get 4
          i32.const 0
          i32.store offset=24
          block  ;; label = @4
            loop  ;; label = @5
              local.get 4
              i32.load offset=24
              local.get 4
              i32.load offset=52
              i32.lt_s
              i32.const 1
              i32.and
              i32.eqz
              br_if 1 (;@4;)
              local.get 0
              i32.load
              local.get 4
              i32.load offset=28
              i32.const 2
              i32.shl
              i32.add
              i32.load
              local.set 1
              local.get 4
              local.get 4
              i32.load offset=32
              local.tee 2
              i32.const 2
              i32.add
              i32.store offset=32
              local.get 2
              local.get 1
              i32.store16
              local.get 4
              local.get 4
              i32.load offset=28
              i32.const 4
              i32.add
              i32.store offset=28
              local.get 4
              local.get 4
              i32.load offset=24
              i32.const 1
              i32.add
              i32.store offset=24
              br 0 (;@5;)
            end
          end
          br 1 (;@2;)
        end
        local.get 4
        i32.load offset=32
        local.get 0
        i32.load offset=48
        local.get 4
        i32.load offset=52
        i32.const 1
        i32.shl
        call 9
        drop
        local.get 4
        local.get 4
        i32.load offset=32
        local.get 4
        i32.load offset=52
        i32.const 1
        i32.shl
        i32.add
        i32.store offset=32
      end
      local.get 4
      local.get 4
      i32.load offset=40
      local.get 4
      i32.load offset=52
      i32.const 1
      i32.shl
      i32.add
      i32.store offset=40
      local.get 4
      local.get 0
      i32.load offset=12
      i32.store offset=20
      local.get 4
      i32.load offset=64
      local.tee 1
      i32.const 2
      i32.gt_u
      drop
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 1
                br_table 0 (;@6;) 1 (;@5;) 2 (;@4;) 3 (;@3;)
              end
              local.get 4
              i32.load offset=36
              local.get 4
              i32.load offset=32
              i32.store offset=4
              local.get 4
              local.get 0
              i32.load offset=20
              i32.store offset=16
              block  ;; label = @6
                loop  ;; label = @7
                  local.get 4
                  i32.load offset=16
                  i32.const 0
                  i32.gt_s
                  i32.const 1
                  i32.and
                  i32.eqz
                  br_if 1 (;@6;)
                  local.get 4
                  local.get 4
                  i32.load offset=20
                  local.tee 1
                  i32.const 4
                  i32.add
                  i32.store offset=20
                  local.get 1
                  i32.load
                  local.set 1
                  local.get 4
                  local.get 4
                  i32.load offset=32
                  local.tee 2
                  i32.const 2
                  i32.add
                  i32.store offset=32
                  local.get 2
                  local.get 1
                  i32.store16
                  local.get 4
                  local.get 4
                  i32.load offset=16
                  i32.const -1
                  i32.add
                  i32.store offset=16
                  br 0 (;@7;)
                end
              end
              br 3 (;@2;)
            end
            local.get 4
            i32.load offset=36
            local.get 4
            i32.load offset=40
            i32.store offset=4
            local.get 4
            i32.load offset=40
            local.get 4
            i32.load offset=20
            local.get 0
            i32.load offset=20
            i32.const 2
            i32.shl
            call 9
            drop
            br 2 (;@2;)
          end
          local.get 4
          i32.load offset=36
          local.get 4
          i32.load offset=40
          i32.store offset=4
          local.get 4
          local.get 0
          i32.load offset=20
          i32.store offset=12
          block  ;; label = @4
            loop  ;; label = @5
              local.get 4
              i32.load offset=12
              i32.const 0
              i32.gt_s
              i32.const 1
              i32.and
              i32.eqz
              br_if 1 (;@4;)
              local.get 4
              local.get 4
              i32.load offset=20
              local.tee 1
              i32.const 4
              i32.add
              i32.store offset=20
              local.get 1
              i32.load
              local.set 1
              local.get 4
              local.get 4
              i32.load offset=40
              local.tee 2
              i32.const 1
              i32.add
              i32.store offset=40
              local.get 2
              local.get 1
              i32.store8
              local.get 4
              local.get 4
              i32.load offset=12
              i32.const -1
              i32.add
              i32.store offset=12
              br 0 (;@5;)
            end
          end
          br 1 (;@2;)
        end
      end
      local.get 0
      call 37
      local.get 4
      local.get 4
      i32.load offset=36
      i32.store offset=76
    end
    local.get 4
    i32.load offset=76
    local.set 0
    local.get 4
    i32.const 80
    i32.add
    global.set 0
    local.get 0)
  (func (;35;) (type 8) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    local.get 0
    i32.store offset=28
    local.get 2
    local.get 1
    i32.store offset=24
    local.get 2
    i32.load offset=28
    local.tee 0
    local.get 0
    i32.load offset=32
    local.get 2
    i32.load offset=24
    i32.and
    i32.store offset=32
    local.get 0
    local.get 0
    i32.load offset=36
    local.get 2
    i32.load offset=24
    i32.and
    i32.store offset=36
    local.get 0
    local.get 0
    i32.load offset=44
    local.get 2
    i32.load offset=24
    i32.and
    i32.store offset=44
    local.get 2
    local.get 0
    i32.load offset=40
    i32.const 4
    i32.shr_s
    i32.store offset=20
    local.get 2
    i32.const 0
    i32.store offset=16
    block  ;; label = @1
      loop  ;; label = @2
        local.get 2
        i32.load offset=16
        local.get 2
        i32.load offset=20
        i32.lt_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        block  ;; label = @3
          local.get 0
          i32.const 52
          i32.add
          local.get 2
          i32.load offset=16
          i32.add
          i32.load8_u
          i32.const 255
          i32.and
          br_if 0 (;@3;)
          local.get 0
          i32.load
          local.get 2
          i32.load offset=16
          i32.const 2
          i32.shl
          i32.add
          local.tee 1
          local.get 1
          i32.load
          local.get 2
          i32.load offset=24
          i32.and
          i32.store
        end
        local.get 2
        local.get 2
        i32.load offset=16
        i32.const 1
        i32.add
        i32.store offset=16
        br 0 (;@2;)
      end
    end
    local.get 2
    i32.const 0
    i32.store offset=12
    block  ;; label = @1
      loop  ;; label = @2
        local.get 2
        i32.load offset=12
        local.get 0
        i32.load offset=20
        i32.lt_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.load offset=12
        local.get 2
        i32.load offset=12
        i32.const 2
        i32.shl
        i32.add
        local.tee 1
        local.get 1
        i32.load
        local.get 2
        i32.load offset=24
        i32.and
        i32.store
        local.get 2
        local.get 2
        i32.load offset=12
        i32.const 1
        i32.add
        i32.store offset=12
        br 0 (;@2;)
      end
    end)
  (func (;36;) (type 4) (param i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 1008
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    local.get 0
    i32.store offset=1000
    local.get 3
    local.get 1
    i32.store offset=996
    local.get 3
    local.get 2
    i32.store offset=992
    local.get 3
    i32.load offset=1000
    local.set 0
    local.get 0
    local.get 0
    i32.const 1114111
    call 38
    i32.store offset=44
    local.get 3
    local.get 0
    call 39
    i32.store offset=988
    local.get 3
    local.get 3
    i32.load offset=988
    i32.const 511
    i32.add
    i32.const -512
    i32.and
    i32.store offset=988
    block  ;; label = @1
      local.get 3
      i32.load offset=988
      i32.const 1114112
      i32.eq
      i32.const 1
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 0
      i32.load offset=32
      i32.store offset=44
    end
    local.get 3
    local.get 3
    i32.load offset=996
    i32.const 4
    i32.shl
    i32.store offset=984
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        i32.load offset=988
        local.get 3
        i32.load offset=984
        i32.lt_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        local.get 3
        i32.load offset=988
        i32.const 4
        i32.shr_s
        i32.store offset=980
        block  ;; label = @3
          loop  ;; label = @4
            local.get 3
            i32.load offset=980
            local.get 3
            i32.load offset=996
            i32.lt_s
            i32.const 1
            i32.and
            i32.eqz
            br_if 1 (;@3;)
            local.get 0
            i32.const 52
            i32.add
            local.get 3
            i32.load offset=980
            i32.add
            i32.const 0
            i32.store8
            local.get 0
            i32.load
            local.get 3
            i32.load offset=980
            i32.const 2
            i32.shl
            i32.add
            local.get 0
            i32.load offset=44
            i32.store
            local.get 3
            local.get 3
            i32.load offset=980
            i32.const 1
            i32.add
            i32.store offset=980
            br 0 (;@4;)
          end
        end
        local.get 0
        local.get 3
        i32.load offset=984
        i32.store offset=40
        br 1 (;@1;)
      end
      local.get 0
      local.get 3
      i32.load offset=988
      i32.store offset=40
    end
    local.get 3
    i32.const 0
    i32.store offset=460
    block  ;; label = @1
      loop  ;; label = @2
        local.get 3
        i32.load offset=460
        i32.const 128
        i32.lt_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        local.get 3
        i32.load offset=460
        call 38
        local.set 1
        local.get 3
        i32.const 464
        i32.add
        local.get 3
        i32.load offset=460
        i32.const 2
        i32.shl
        i32.add
        local.get 1
        i32.store
        local.get 3
        local.get 3
        i32.load offset=460
        i32.const 1
        i32.add
        i32.store offset=460
        br 0 (;@2;)
      end
    end
    local.get 3
    i32.const 64
    i32.add
    call 40
    local.set 1
    local.get 3
    local.get 0
    local.get 3
    i32.load offset=996
    local.get 1
    call 41
    i32.store offset=60
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        i32.load offset=60
        i32.const 0
        i32.lt_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.load offset=992
        i32.const 7
        i32.store
        local.get 3
        i32.const 0
        i32.store offset=1004
        br 1 (;@1;)
      end
      local.get 3
      local.get 3
      i32.load offset=60
      i32.const 2
      i32.shl
      call 11
      i32.store offset=56
      block  ;; label = @2
        local.get 3
        i32.load offset=56
        i32.const 0
        i32.eq
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.load offset=992
        i32.const 7
        i32.store
        local.get 3
        i32.const 0
        i32.store offset=1004
        br 1 (;@1;)
      end
      local.get 3
      i32.load offset=56
      local.get 3
      i32.const 464
      i32.add
      i32.const 512
      call 9
      drop
      local.get 3
      local.get 1
      call 42
      i32.store offset=52
      local.get 3
      i32.const 24
      i32.add
      call 43
      local.set 1
      local.get 0
      local.get 3
      i32.load offset=996
      local.get 3
      i32.load offset=56
      local.get 3
      i32.load offset=60
      local.get 3
      i32.load offset=52
      local.get 1
      local.get 3
      i32.load offset=992
      call 44
      local.set 2
      local.get 3
      local.get 2
      i32.store offset=20
      block  ;; label = @2
        block  ;; label = @3
          local.get 3
          i32.load offset=992
          i32.load
          call 15
          i32.const 255
          i32.and
          i32.const 0
          i32.const 255
          i32.and
          i32.ne
          i32.const 1
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          i32.const 0
          i32.store offset=1004
          local.get 3
          i32.const 1
          i32.store offset=8
          br 1 (;@2;)
        end
        local.get 0
        i32.load offset=12
        call 12
        local.get 0
        local.get 3
        i32.load offset=56
        i32.store offset=12
        local.get 0
        local.get 3
        i32.load offset=60
        i32.store offset=16
        local.get 0
        local.get 3
        i32.load offset=20
        i32.store offset=20
        block  ;; label = @3
          local.get 0
          i32.load offset=20
          i32.const 262159
          i32.gt_s
          i32.const 1
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          i32.load offset=992
          i32.const 8
          i32.store
          local.get 3
          i32.const 0
          i32.store offset=1004
          local.get 3
          i32.const 1
          i32.store offset=8
          br 1 (;@2;)
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 3
            i32.load offset=52
            i32.const 0
            i32.ge_s
            i32.const 1
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            local.get 0
            i32.load
            local.get 3
            i32.load offset=52
            i32.const 2
            i32.shl
            i32.add
            i32.load
            i32.store offset=24
            local.get 0
            local.get 0
            i32.load offset=12
            local.get 0
            i32.load offset=24
            i32.const 2
            i32.shl
            i32.add
            i32.load
            i32.store offset=32
            br 1 (;@3;)
          end
          local.get 0
          i32.const 1048575
          i32.store offset=24
        end
        local.get 0
        local.get 3
        i32.load offset=996
        local.get 1
        local.get 3
        i32.load offset=992
        call 45
        local.set 2
        local.get 3
        local.get 2
        i32.store offset=4
        local.get 0
        local.get 3
        i32.load offset=988
        i32.store offset=40
        local.get 3
        local.get 3
        i32.load offset=4
        i32.store offset=1004
        local.get 3
        i32.const 1
        i32.store offset=8
      end
      local.get 1
      call 46
      drop
    end
    local.get 3
    i32.load offset=1004
    local.set 0
    local.get 3
    i32.const 1008
    i32.add
    global.set 0
    local.get 0)
  (func (;37;) (type 2) (param i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    local.get 0
    i32.store offset=12
    local.get 1
    i32.load offset=12
    local.tee 0
    i32.const -1
    i32.store offset=24
    local.get 0
    i32.const -1
    i32.store offset=8
    local.get 0
    i32.const 0
    i32.store offset=20
    local.get 0
    local.get 0
    i32.load offset=28
    local.tee 2
    i32.store offset=32
    local.get 0
    local.get 2
    i32.store offset=44
    local.get 0
    i32.const 0
    i32.store offset=40
    local.get 0
    i32.load offset=48
    call 12
    local.get 0
    i32.const 0
    i32.store offset=48
    local.get 1
    i32.const 16
    i32.add
    global.set 0)
  (func (;38;) (type 3) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    local.get 0
    i32.store offset=8
    local.get 2
    local.get 1
    i32.store offset=4
    local.get 2
    i32.load offset=8
    local.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.load offset=4
        i32.const 1114111
        i32.gt_u
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 0
        i32.load offset=36
        i32.store offset=12
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 2
        i32.load offset=4
        local.get 0
        i32.load offset=40
        i32.ge_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 0
        i32.load offset=44
        i32.store offset=12
        br 1 (;@1;)
      end
      local.get 2
      local.get 2
      i32.load offset=4
      i32.const 4
      i32.shr_s
      i32.store
      block  ;; label = @2
        local.get 0
        i32.const 52
        i32.add
        local.get 2
        i32.load
        i32.add
        i32.load8_u
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        local.get 2
        local.get 0
        i32.load
        local.get 2
        i32.load
        i32.const 2
        i32.shl
        i32.add
        i32.load
        i32.store offset=12
        br 1 (;@1;)
      end
      local.get 2
      local.get 0
      i32.load offset=12
      local.get 0
      i32.load
      local.get 2
      i32.load
      i32.const 2
      i32.shl
      i32.add
      i32.load
      local.get 2
      i32.load offset=4
      i32.const 15
      i32.and
      i32.add
      i32.const 2
      i32.shl
      i32.add
      i32.load
      i32.store offset=12
    end
    local.get 2
    i32.load offset=12)
  (func (;39;) (type 1) (param i32) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 1
    local.get 0
    i32.store offset=24
    local.get 1
    local.get 1
    i32.load offset=24
    local.tee 0
    i32.load offset=40
    i32.const 4
    i32.shr_s
    i32.store offset=20
    block  ;; label = @1
      block  ;; label = @2
        loop  ;; label = @3
          local.get 1
          i32.load offset=20
          i32.const 0
          i32.gt_s
          i32.const 1
          i32.and
          i32.eqz
          br_if 1 (;@2;)
          local.get 1
          local.get 1
          i32.load offset=20
          i32.const -1
          i32.add
          local.tee 2
          i32.store offset=20
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.const 52
              i32.add
              local.get 2
              i32.add
              i32.load8_u
              i32.const 255
              i32.and
              br_if 0 (;@5;)
              local.get 1
              local.get 0
              i32.load
              local.get 1
              i32.load offset=20
              i32.const 2
              i32.shl
              i32.add
              i32.load
              local.get 0
              i32.load offset=44
              i32.eq
              i32.const 1
              i32.and
              i32.store8 offset=19
              br 1 (;@4;)
            end
            local.get 1
            local.get 0
            i32.load offset=12
            local.get 0
            i32.load
            local.get 1
            i32.load offset=20
            i32.const 2
            i32.shl
            i32.add
            i32.load
            i32.const 2
            i32.shl
            i32.add
            i32.store offset=12
            local.get 1
            i32.const 0
            i32.store offset=8
            block  ;; label = @5
              loop  ;; label = @6
                block  ;; label = @7
                  local.get 1
                  i32.load offset=8
                  i32.const 16
                  i32.eq
                  i32.const 1
                  i32.and
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 1
                  i32.const 1
                  i32.store8 offset=19
                  br 2 (;@5;)
                end
                block  ;; label = @7
                  local.get 1
                  i32.load offset=12
                  local.get 1
                  i32.load offset=8
                  i32.const 2
                  i32.shl
                  i32.add
                  i32.load
                  local.get 0
                  i32.load offset=44
                  i32.ne
                  i32.const 1
                  i32.and
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 1
                  i32.const 0
                  i32.store8 offset=19
                  br 2 (;@5;)
                end
                local.get 1
                local.get 1
                i32.load offset=8
                i32.const 1
                i32.add
                i32.store offset=8
                br 0 (;@6;)
              end
            end
          end
          block  ;; label = @4
            local.get 1
            i32.load8_u offset=19
            i32.const 1
            i32.and
            br_if 0 (;@4;)
            local.get 1
            local.get 1
            i32.load offset=20
            i32.const 1
            i32.add
            i32.const 4
            i32.shl
            i32.store offset=28
            br 3 (;@1;)
          end
          br 0 (;@3;)
        end
      end
      local.get 1
      i32.const 0
      i32.store offset=28
    end
    local.get 1
    i32.load offset=28)
  (func (;40;) (type 1) (param i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    local.get 0
    i32.store offset=12
    local.get 1
    i32.load offset=12
    local.tee 0
    i32.const 0
    i32.store
    local.get 0
    i32.const -1
    i32.store offset=4
    local.get 0)
  (func (;41;) (type 4) (param i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 80
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    local.get 0
    i32.store offset=72
    local.get 3
    local.get 1
    i32.store offset=68
    local.get 3
    local.get 2
    i32.store offset=64
    local.get 3
    i32.load offset=72
    local.set 0
    local.get 3
    i32.const 128
    i32.store offset=60
    local.get 3
    local.get 3
    i32.load offset=60
    i32.const 16
    i32.add
    i32.store offset=60
    local.get 3
    local.get 3
    i32.load offset=60
    i32.const 4
    i32.add
    i32.store offset=60
    local.get 3
    local.get 0
    i32.load offset=40
    i32.const 4
    i32.shr_s
    i32.store offset=56
    local.get 3
    i32.const 64
    i32.store offset=52
    local.get 3
    i32.const 4
    i32.store offset=48
    local.get 3
    i32.const 0
    i32.store offset=44
    block  ;; label = @1
      block  ;; label = @2
        loop  ;; label = @3
          local.get 3
          i32.load offset=44
          local.get 3
          i32.load offset=56
          i32.lt_s
          i32.const 1
          i32.and
          i32.eqz
          br_if 1 (;@2;)
          block  ;; label = @4
            local.get 3
            i32.load offset=44
            local.get 3
            i32.load offset=68
            i32.eq
            i32.const 1
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 3
            i32.const 16
            i32.store offset=52
            local.get 3
            i32.const 1
            i32.store offset=48
          end
          local.get 3
          local.get 0
          i32.load
          local.get 3
          i32.load offset=44
          i32.const 2
          i32.shl
          i32.add
          i32.load
          i32.store offset=40
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.const 52
                i32.add
                local.get 3
                i32.load offset=44
                i32.add
                i32.load8_u
                i32.const 255
                i32.and
                i32.const 1
                i32.eq
                i32.const 1
                i32.and
                i32.eqz
                br_if 0 (;@6;)
                local.get 3
                local.get 0
                i32.load offset=12
                local.get 3
                i32.load offset=40
                i32.const 2
                i32.shl
                i32.add
                i32.store offset=36
                local.get 3
                local.get 3
                i32.load offset=36
                i32.load
                i32.store offset=40
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 3
                    i32.load offset=36
                    i32.const 4
                    i32.add
                    local.get 3
                    i32.load offset=52
                    i32.const 1
                    i32.sub
                    local.get 3
                    i32.load offset=40
                    call 47
                    i32.const 1
                    i32.and
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 0
                    i32.const 52
                    i32.add
                    local.get 3
                    i32.load offset=44
                    i32.add
                    i32.const 0
                    i32.store8
                    local.get 0
                    i32.load
                    local.get 3
                    i32.load offset=44
                    i32.const 2
                    i32.shl
                    i32.add
                    local.get 3
                    i32.load offset=40
                    i32.store
                    br 1 (;@7;)
                  end
                  local.get 3
                  local.get 3
                  i32.load offset=60
                  local.get 3
                  i32.load offset=52
                  i32.add
                  i32.store offset=60
                  br 3 (;@4;)
                end
                br 1 (;@5;)
              end
              block  ;; label = @6
                local.get 3
                i32.load offset=48
                i32.const 1
                i32.gt_s
                i32.const 1
                i32.and
                i32.eqz
                br_if 0 (;@6;)
                local.get 3
                i32.const 1
                i32.store8 offset=35
                local.get 3
                local.get 3
                i32.load offset=44
                local.get 3
                i32.load offset=48
                i32.add
                i32.store offset=28
                local.get 3
                local.get 3
                i32.load offset=44
                i32.const 1
                i32.add
                i32.store offset=24
                block  ;; label = @7
                  block  ;; label = @8
                    loop  ;; label = @9
                      local.get 3
                      i32.load offset=24
                      local.get 3
                      i32.load offset=28
                      i32.lt_s
                      i32.const 1
                      i32.and
                      i32.eqz
                      br_if 1 (;@8;)
                      block  ;; label = @10
                        local.get 0
                        i32.load
                        local.get 3
                        i32.load offset=24
                        i32.const 2
                        i32.shl
                        i32.add
                        i32.load
                        local.get 3
                        i32.load offset=40
                        i32.ne
                        i32.const 1
                        i32.and
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 3
                        i32.const 0
                        i32.store8 offset=35
                        br 3 (;@7;)
                      end
                      local.get 3
                      local.get 3
                      i32.load offset=24
                      i32.const 1
                      i32.add
                      i32.store offset=24
                      br 0 (;@9;)
                    end
                  end
                end
                block  ;; label = @7
                  local.get 3
                  i32.load8_u offset=35
                  i32.const 1
                  i32.and
                  br_if 0 (;@7;)
                  block  ;; label = @8
                    local.get 0
                    local.get 3
                    i32.load offset=44
                    call 30
                    i32.const 0
                    i32.lt_s
                    i32.const 1
                    i32.and
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 3
                    i32.const -1
                    i32.store offset=76
                    br 7 (;@1;)
                  end
                  local.get 3
                  local.get 3
                  i32.load offset=60
                  local.get 3
                  i32.load offset=52
                  i32.add
                  i32.store offset=60
                  br 3 (;@4;)
                end
              end
            end
            local.get 3
            local.get 3
            i32.load offset=64
            local.get 3
            i32.load offset=44
            local.get 3
            i32.load offset=48
            local.get 3
            i32.load offset=40
            call 48
            i32.store offset=20
            block  ;; label = @5
              local.get 3
              i32.load offset=20
              i32.const -2
              i32.eq
              i32.const 1
              i32.and
              i32.eqz
              br_if 0 (;@5;)
              local.get 3
              i32.const 4
              i32.store offset=16
              local.get 3
              i32.const 0
              i32.store offset=12
              block  ;; label = @6
                loop  ;; label = @7
                  block  ;; label = @8
                    local.get 3
                    i32.load offset=12
                    local.get 3
                    i32.load offset=44
                    i32.eq
                    i32.const 1
                    i32.and
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 3
                    i32.load offset=64
                    local.get 3
                    i32.load offset=44
                    local.get 3
                    i32.load offset=48
                    local.get 3
                    i32.load offset=40
                    call 49
                    br 2 (;@6;)
                  end
                  block  ;; label = @8
                    local.get 3
                    i32.load offset=12
                    local.get 3
                    i32.load offset=68
                    i32.eq
                    i32.const 1
                    i32.and
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 3
                    i32.const 1
                    i32.store offset=16
                  end
                  block  ;; label = @8
                    local.get 0
                    i32.const 52
                    i32.add
                    local.get 3
                    i32.load offset=12
                    i32.add
                    i32.load8_u
                    i32.const 255
                    i32.and
                    br_if 0 (;@8;)
                    local.get 0
                    i32.load
                    local.get 3
                    i32.load offset=12
                    i32.const 2
                    i32.shl
                    i32.add
                    i32.load
                    local.get 3
                    i32.load offset=40
                    i32.eq
                    i32.const 1
                    i32.and
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 3
                    i32.load offset=64
                    local.get 3
                    i32.load offset=12
                    local.get 3
                    i32.load offset=16
                    local.get 3
                    i32.load offset=48
                    i32.add
                    local.get 3
                    i32.load offset=40
                    call 49
                    local.get 3
                    local.get 3
                    i32.load offset=12
                    i32.store offset=20
                    br 2 (;@6;)
                  end
                  local.get 3
                  local.get 3
                  i32.load offset=12
                  local.get 3
                  i32.load offset=16
                  i32.add
                  i32.store offset=12
                  br 0 (;@7;)
                end
              end
            end
            block  ;; label = @5
              block  ;; label = @6
                local.get 3
                i32.load offset=20
                i32.const 0
                i32.ge_s
                i32.const 1
                i32.and
                i32.eqz
                br_if 0 (;@6;)
                local.get 0
                i32.const 52
                i32.add
                local.get 3
                i32.load offset=44
                i32.add
                i32.const 2
                i32.store8
                local.get 0
                i32.load
                local.get 3
                i32.load offset=44
                i32.const 2
                i32.shl
                i32.add
                local.get 3
                i32.load offset=20
                i32.store
                br 1 (;@5;)
              end
              local.get 3
              local.get 3
              i32.load offset=60
              local.get 3
              i32.load offset=52
              i32.add
              i32.store offset=60
            end
          end
          local.get 3
          local.get 3
          i32.load offset=44
          local.get 3
          i32.load offset=48
          i32.add
          i32.store offset=44
          br 0 (;@3;)
        end
      end
      local.get 3
      local.get 3
      i32.load offset=60
      i32.store offset=76
    end
    local.get 3
    i32.load offset=76
    local.set 0
    local.get 3
    i32.const 80
    i32.add
    global.set 0
    local.get 0)
  (func (;42;) (type 1) (param i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 1
    local.get 0
    i32.store offset=24
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.load offset=24
        local.tee 0
        i32.load
        br_if 0 (;@2;)
        local.get 1
        i32.const -1
        i32.store offset=28
        br 1 (;@1;)
      end
      local.get 1
      i32.const -1
      i32.store offset=20
      local.get 1
      i32.const 0
      i32.store offset=16
      local.get 1
      i32.const 0
      i32.store offset=12
      block  ;; label = @2
        loop  ;; label = @3
          local.get 1
          i32.load offset=12
          local.get 0
          i32.load
          i32.lt_s
          i32.const 1
          i32.and
          i32.eqz
          br_if 1 (;@2;)
          block  ;; label = @4
            local.get 0
            i32.const 264
            i32.add
            local.get 1
            i32.load offset=12
            i32.const 2
            i32.shl
            i32.add
            i32.load
            local.get 1
            i32.load offset=16
            i32.gt_s
            i32.const 1
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 1
            local.get 1
            i32.load offset=12
            i32.store offset=20
            local.get 1
            local.get 0
            i32.const 264
            i32.add
            local.get 1
            i32.load offset=12
            i32.const 2
            i32.shl
            i32.add
            i32.load
            i32.store offset=16
          end
          local.get 1
          local.get 1
          i32.load offset=12
          i32.const 1
          i32.add
          i32.store offset=12
          br 0 (;@3;)
        end
      end
      local.get 1
      local.get 0
      i32.const 8
      i32.add
      local.get 1
      i32.load offset=20
      i32.const 2
      i32.shl
      i32.add
      i32.load
      i32.store offset=28
    end
    local.get 1
    i32.load offset=28)
  (func (;43;) (type 1) (param i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    local.get 0
    i32.store offset=12
    local.get 1
    i32.load offset=12
    local.tee 0
    i32.const 0
    i32.store
    local.get 0
    i32.const 0
    i32.store offset=4
    local.get 0
    i32.const 0
    i32.store offset=8
    local.get 0
    i32.const 0
    i32.store offset=12
    local.get 0
    i32.const 0
    i32.store offset=16
    local.get 0
    i32.const 0
    i32.store offset=20
    local.get 0)
  (func (;44;) (type 9) (param i32 i32 i32 i32 i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 96
    i32.sub
    local.tee 7
    global.set 0
    local.get 7
    local.get 0
    i32.store offset=88
    local.get 7
    local.get 1
    i32.store offset=84
    local.get 7
    local.get 2
    i32.store offset=80
    local.get 7
    local.get 3
    i32.store offset=76
    local.get 7
    local.get 4
    i32.store offset=72
    local.get 7
    local.get 5
    i32.store offset=68
    local.get 7
    local.get 6
    i32.store offset=64
    local.get 7
    i32.load offset=88
    local.set 0
    local.get 7
    i32.const 0
    i32.store offset=60
    local.get 7
    i32.const 0
    i32.store offset=56
    block  ;; label = @1
      loop  ;; label = @2
        local.get 7
        i32.load offset=60
        i32.const 128
        i32.lt_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.load
        local.get 7
        i32.load offset=56
        i32.const 2
        i32.shl
        i32.add
        local.get 7
        i32.load offset=60
        i32.store
        local.get 7
        local.get 7
        i32.load offset=60
        i32.const 64
        i32.add
        i32.store offset=60
        local.get 7
        local.get 7
        i32.load offset=56
        i32.const 4
        i32.add
        i32.store offset=56
        br 0 (;@2;)
      end
    end
    local.get 7
    i32.const 64
    i32.store offset=52
    block  ;; label = @1
      block  ;; label = @2
        local.get 7
        i32.load offset=68
        local.get 7
        i32.load offset=76
        local.get 7
        i32.load offset=52
        call 50
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 7
        i32.load offset=64
        i32.const 7
        i32.store
        local.get 7
        i32.const 0
        i32.store offset=92
        br 1 (;@1;)
      end
      local.get 7
      i32.load offset=68
      local.get 7
      i32.load offset=80
      i32.const 0
      local.get 7
      i32.load offset=60
      call 51
      local.get 7
      local.get 0
      i32.load offset=40
      i32.const 4
      i32.shr_s
      i32.store offset=48
      local.get 7
      i32.const 4
      i32.store offset=44
      local.get 7
      i32.const 0
      i32.store offset=40
      local.get 7
      i32.const 8
      i32.store offset=36
      block  ;; label = @2
        loop  ;; label = @3
          local.get 7
          i32.load offset=36
          local.get 7
          i32.load offset=48
          i32.lt_s
          i32.const 1
          i32.and
          i32.eqz
          br_if 1 (;@2;)
          block  ;; label = @4
            local.get 7
            i32.load offset=36
            local.get 7
            i32.load offset=84
            i32.eq
            i32.const 1
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 7
            i32.const 16
            i32.store offset=52
            local.get 7
            i32.const 1
            i32.store offset=44
            local.get 7
            local.get 7
            i32.load offset=60
            i32.store offset=40
            block  ;; label = @5
              local.get 7
              i32.load offset=68
              local.get 7
              i32.load offset=76
              local.get 7
              i32.load offset=52
              call 50
              i32.const 1
              i32.and
              br_if 0 (;@5;)
              local.get 7
              i32.load offset=64
              i32.const 7
              i32.store
              local.get 7
              i32.const 0
              i32.store offset=92
              br 4 (;@1;)
            end
            local.get 7
            i32.load offset=68
            local.get 7
            i32.load offset=80
            i32.const 0
            local.get 7
            i32.load offset=60
            call 51
          end
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.const 52
              i32.add
              local.get 7
              i32.load offset=36
              i32.add
              i32.load8_u
              i32.const 255
              i32.and
              br_if 0 (;@5;)
              local.get 7
              local.get 0
              i32.load
              local.get 7
              i32.load offset=36
              i32.const 2
              i32.shl
              i32.add
              i32.load
              i32.store offset=32
              local.get 7
              local.get 7
              i32.load offset=68
              local.get 7
              i32.load offset=80
              local.get 7
              i32.load offset=32
              call 52
              i32.store offset=28
              loop  ;; label = @6
                i32.const 0
                local.set 1
                block  ;; label = @7
                  local.get 7
                  i32.load offset=28
                  i32.const 0
                  i32.ge_s
                  i32.const 1
                  i32.and
                  i32.eqz
                  br_if 0 (;@7;)
                  i32.const 0
                  local.set 1
                  local.get 7
                  i32.load offset=36
                  local.get 7
                  i32.load offset=72
                  i32.eq
                  i32.const 1
                  i32.and
                  i32.eqz
                  br_if 0 (;@7;)
                  i32.const 0
                  local.set 1
                  local.get 7
                  i32.load offset=36
                  local.get 7
                  i32.load offset=84
                  i32.ge_s
                  i32.const 1
                  i32.and
                  i32.eqz
                  br_if 0 (;@7;)
                  i32.const 0
                  local.set 1
                  local.get 7
                  i32.load offset=28
                  local.get 7
                  i32.load offset=40
                  i32.lt_s
                  i32.const 1
                  i32.and
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 7
                  i32.load offset=28
                  local.get 0
                  i32.load
                  local.get 7
                  i32.load offset=84
                  call 53
                  local.set 1
                end
                block  ;; label = @7
                  local.get 1
                  i32.const 1
                  i32.and
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 7
                  local.get 7
                  i32.load offset=80
                  local.get 7
                  i32.load offset=28
                  i32.const 1
                  i32.add
                  local.get 7
                  i32.load offset=60
                  local.get 7
                  i32.load offset=32
                  local.get 7
                  i32.load offset=52
                  call 54
                  i32.store offset=28
                  br 1 (;@6;)
                end
              end
              block  ;; label = @6
                block  ;; label = @7
                  local.get 7
                  i32.load offset=28
                  i32.const 0
                  i32.ge_s
                  i32.const 1
                  i32.and
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 0
                  i32.load
                  local.get 7
                  i32.load offset=36
                  i32.const 2
                  i32.shl
                  i32.add
                  local.get 7
                  i32.load offset=28
                  i32.store
                  br 1 (;@6;)
                end
                local.get 7
                local.get 7
                i32.load offset=80
                local.get 7
                i32.load offset=60
                local.get 7
                i32.load offset=32
                local.get 7
                i32.load offset=52
                call 55
                i32.store offset=28
                local.get 0
                i32.load
                local.get 7
                i32.load offset=36
                i32.const 2
                i32.shl
                i32.add
                local.get 7
                i32.load offset=60
                local.get 7
                i32.load offset=28
                i32.sub
                i32.store
                local.get 7
                local.get 7
                i32.load offset=60
                i32.store offset=24
                block  ;; label = @7
                  loop  ;; label = @8
                    local.get 7
                    i32.load offset=28
                    local.get 7
                    i32.load offset=52
                    i32.lt_s
                    i32.const 1
                    i32.and
                    i32.eqz
                    br_if 1 (;@7;)
                    local.get 7
                    i32.load offset=32
                    local.set 1
                    local.get 7
                    i32.load offset=80
                    local.set 2
                    local.get 7
                    local.get 7
                    i32.load offset=60
                    local.tee 3
                    i32.const 1
                    i32.add
                    i32.store offset=60
                    local.get 2
                    local.get 3
                    i32.const 2
                    i32.shl
                    i32.add
                    local.get 1
                    i32.store
                    local.get 7
                    local.get 7
                    i32.load offset=28
                    i32.const 1
                    i32.add
                    i32.store offset=28
                    br 0 (;@8;)
                  end
                end
                local.get 7
                i32.load offset=68
                local.get 7
                i32.load offset=80
                local.get 7
                i32.load offset=24
                local.get 7
                i32.load offset=60
                call 51
              end
              br 1 (;@4;)
            end
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.const 52
                i32.add
                local.get 7
                i32.load offset=36
                i32.add
                i32.load8_u
                i32.const 255
                i32.and
                i32.const 1
                i32.eq
                i32.const 1
                i32.and
                i32.eqz
                br_if 0 (;@6;)
                local.get 7
                local.get 0
                i32.load offset=12
                local.get 0
                i32.load
                local.get 7
                i32.load offset=36
                i32.const 2
                i32.shl
                i32.add
                i32.load
                i32.const 2
                i32.shl
                i32.add
                i32.store offset=20
                local.get 7
                local.get 7
                i32.load offset=68
                local.get 7
                i32.load offset=80
                local.get 7
                i32.load offset=20
                call 56
                i32.store offset=16
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 7
                    i32.load offset=16
                    i32.const 0
                    i32.ge_s
                    i32.const 1
                    i32.and
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 0
                    i32.load
                    local.get 7
                    i32.load offset=36
                    i32.const 2
                    i32.shl
                    i32.add
                    local.get 7
                    i32.load offset=16
                    i32.store
                    br 1 (;@7;)
                  end
                  local.get 7
                  local.get 7
                  i32.load offset=80
                  local.get 7
                  i32.load offset=60
                  local.get 7
                  i32.load offset=20
                  local.get 7
                  i32.load offset=52
                  call 57
                  i32.store offset=16
                  local.get 0
                  i32.load
                  local.get 7
                  i32.load offset=36
                  i32.const 2
                  i32.shl
                  i32.add
                  local.get 7
                  i32.load offset=60
                  local.get 7
                  i32.load offset=16
                  i32.sub
                  i32.store
                  local.get 7
                  local.get 7
                  i32.load offset=60
                  i32.store offset=12
                  block  ;; label = @8
                    loop  ;; label = @9
                      local.get 7
                      i32.load offset=16
                      local.get 7
                      i32.load offset=52
                      i32.lt_s
                      i32.const 1
                      i32.and
                      i32.eqz
                      br_if 1 (;@8;)
                      local.get 7
                      i32.load offset=20
                      local.set 1
                      local.get 7
                      local.get 7
                      i32.load offset=16
                      local.tee 2
                      i32.const 1
                      i32.add
                      i32.store offset=16
                      local.get 1
                      local.get 2
                      i32.const 2
                      i32.shl
                      i32.add
                      i32.load
                      local.set 1
                      local.get 7
                      i32.load offset=80
                      local.set 2
                      local.get 7
                      local.get 7
                      i32.load offset=60
                      local.tee 3
                      i32.const 1
                      i32.add
                      i32.store offset=60
                      local.get 2
                      local.get 3
                      i32.const 2
                      i32.shl
                      i32.add
                      local.get 1
                      i32.store
                      br 0 (;@9;)
                    end
                  end
                  local.get 7
                  i32.load offset=68
                  local.get 7
                  i32.load offset=80
                  local.get 7
                  i32.load offset=12
                  local.get 7
                  i32.load offset=60
                  call 51
                end
                br 1 (;@5;)
              end
              local.get 7
              local.get 0
              i32.load
              local.get 7
              i32.load offset=36
              i32.const 2
              i32.shl
              i32.add
              i32.load
              i32.store offset=8
              local.get 0
              i32.load
              local.get 7
              i32.load offset=36
              i32.const 2
              i32.shl
              i32.add
              local.get 0
              i32.load
              local.get 7
              i32.load offset=8
              i32.const 2
              i32.shl
              i32.add
              i32.load
              i32.store
            end
          end
          local.get 7
          local.get 7
          i32.load offset=36
          local.get 7
          i32.load offset=44
          i32.add
          i32.store offset=36
          br 0 (;@3;)
        end
      end
      local.get 7
      local.get 7
      i32.load offset=60
      i32.store offset=92
    end
    local.get 7
    i32.load offset=92
    local.set 0
    local.get 7
    i32.const 96
    i32.add
    global.set 0
    local.get 0)
  (func (;45;) (type 6) (param i32 i32 i32 i32) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 6656
    i32.sub
    local.tee 4
    global.set 0
    local.get 4
    local.get 0
    i32.store offset=6648
    local.get 4
    local.get 1
    i32.store offset=6644
    local.get 4
    local.get 2
    i32.store offset=6640
    local.get 4
    local.get 3
    i32.store offset=6636
    local.get 4
    i32.load offset=6648
    local.set 0
    local.get 4
    local.get 4
    i32.load offset=6644
    i32.const 2
    i32.shr_s
    i32.store offset=6632
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load offset=40
        i32.const 6
        i32.shr_s
        local.get 4
        i32.load offset=6632
        i32.le_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.const 32767
        i32.store offset=8
        local.get 4
        local.get 4
        i32.load offset=6632
        i32.store offset=6652
        br 1 (;@1;)
      end
      local.get 4
      i32.const -1
      i32.store offset=4572
      local.get 4
      i32.const 0
      i32.store offset=4568
      local.get 4
      i32.const 0
      i32.store offset=4564
      block  ;; label = @2
        loop  ;; label = @3
          local.get 4
          i32.load offset=4568
          local.get 4
          i32.load offset=6644
          i32.lt_s
          i32.const 1
          i32.and
          i32.eqz
          br_if 1 (;@2;)
          local.get 4
          local.get 0
          i32.load
          local.get 4
          i32.load offset=4568
          i32.const 2
          i32.shl
          i32.add
          i32.load
          i32.store offset=4560
          local.get 4
          i32.const 4576
          i32.add
          local.get 4
          i32.load offset=4564
          i32.const 1
          i32.shl
          i32.add
          local.get 4
          i32.load offset=4560
          i32.store16
          block  ;; label = @4
            block  ;; label = @5
              local.get 4
              i32.load offset=4560
              local.get 0
              i32.load offset=24
              i32.eq
              i32.const 1
              i32.and
              i32.eqz
              br_if 0 (;@5;)
              block  ;; label = @6
                block  ;; label = @7
                  local.get 4
                  i32.load offset=4572
                  i32.const 0
                  i32.lt_s
                  i32.const 1
                  i32.and
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 4
                  local.get 4
                  i32.load offset=4564
                  i32.store offset=4572
                  br 1 (;@6;)
                end
                block  ;; label = @7
                  local.get 0
                  i32.load offset=8
                  i32.const 0
                  i32.lt_s
                  i32.const 1
                  i32.and
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 4
                  i32.load offset=4564
                  local.get 4
                  i32.load offset=4572
                  i32.sub
                  i32.const 1
                  i32.add
                  i32.const 32
                  i32.eq
                  i32.const 1
                  i32.and
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 0
                  local.get 4
                  i32.load offset=4572
                  i32.store offset=8
                end
              end
              br 1 (;@4;)
            end
            local.get 4
            i32.const -1
            i32.store offset=4572
          end
          local.get 4
          local.get 4
          i32.load offset=4568
          i32.const 4
          i32.add
          i32.store offset=4556
          block  ;; label = @4
            loop  ;; label = @5
              local.get 4
              local.get 4
              i32.load offset=4568
              i32.const 1
              i32.add
              local.tee 1
              i32.store offset=4568
              local.get 1
              local.get 4
              i32.load offset=4556
              i32.lt_s
              i32.const 1
              i32.and
              i32.eqz
              br_if 1 (;@4;)
              local.get 4
              local.get 4
              i32.load offset=4560
              i32.const 16
              i32.add
              i32.store offset=4560
              local.get 0
              i32.load
              local.get 4
              i32.load offset=4568
              i32.const 2
              i32.shl
              i32.add
              local.get 4
              i32.load offset=4560
              i32.store
              br 0 (;@5;)
            end
          end
          local.get 4
          local.get 4
          i32.load offset=4564
          i32.const 1
          i32.add
          i32.store offset=4564
          br 0 (;@3;)
        end
      end
      block  ;; label = @2
        local.get 4
        i32.load offset=6640
        local.get 4
        i32.load offset=6632
        i32.const 32
        call 50
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 4
        i32.load offset=6636
        i32.const 7
        i32.store
        local.get 4
        i32.const 0
        i32.store offset=6652
        br 1 (;@1;)
      end
      local.get 4
      i32.load offset=6640
      local.get 4
      i32.const 4576
      i32.add
      i32.const 0
      i32.const 0
      local.get 4
      i32.load offset=6632
      call 58
      local.get 4
      i32.const 0
      i32.store offset=4552
      local.get 4
      local.get 0
      i32.load offset=8
      i32.store offset=4572
      local.get 4
      i32.const 0
      i32.store8 offset=4551
      local.get 4
      i32.const 0
      i32.const 4096
      local.get 4
      i32.load offset=6644
      i32.const 4096
      i32.lt_s
      i32.const 1
      i32.and
      select
      i32.store offset=4544
      local.get 4
      local.get 0
      i32.load offset=40
      i32.const 4
      i32.shr_s
      i32.store offset=4540
      local.get 4
      local.get 4
      i32.load offset=4544
      i32.store offset=4536
      block  ;; label = @2
        loop  ;; label = @3
          local.get 4
          i32.load offset=4536
          local.get 4
          i32.load offset=4540
          i32.lt_s
          i32.const 1
          i32.and
          i32.eqz
          br_if 1 (;@2;)
          local.get 4
          local.get 4
          i32.load offset=4536
          i32.store offset=4532
          local.get 4
          local.get 4
          i32.load offset=4536
          i32.const 32
          i32.add
          i32.store offset=4528
          local.get 4
          i32.const 0
          i32.store offset=4524
          local.get 4
          i32.const 1
          i32.store8 offset=4523
          loop  ;; label = @4
            local.get 4
            local.get 0
            i32.load
            local.get 4
            i32.load offset=4532
            i32.const 2
            i32.shl
            i32.add
            i32.load
            i32.store offset=4516
            local.get 4
            local.get 4
            i32.load offset=4524
            local.get 4
            i32.load offset=4516
            i32.or
            i32.store offset=4524
            block  ;; label = @5
              local.get 4
              i32.load offset=4516
              local.get 0
              i32.load offset=24
              i32.ne
              i32.const 1
              i32.and
              i32.eqz
              br_if 0 (;@5;)
              local.get 4
              i32.const 0
              i32.store8 offset=4523
            end
            local.get 4
            local.get 4
            i32.load offset=4532
            i32.const 1
            i32.add
            local.tee 1
            i32.store offset=4532
            local.get 1
            local.get 4
            i32.load offset=4528
            i32.lt_s
            i32.const 1
            i32.and
            br_if 0 (;@4;)
          end
          block  ;; label = @4
            block  ;; label = @5
              local.get 4
              i32.load8_u offset=4523
              i32.const 1
              i32.and
              i32.eqz
              br_if 0 (;@5;)
              local.get 0
              i32.const 52
              i32.add
              local.get 4
              i32.load offset=4536
              i32.add
              i32.const 0
              i32.store8
              block  ;; label = @6
                local.get 4
                i32.load offset=4572
                i32.const 0
                i32.lt_s
                i32.const 1
                i32.and
                i32.eqz
                br_if 0 (;@6;)
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 4
                    i32.load offset=4524
                    i32.const 65535
                    i32.le_u
                    i32.const 1
                    i32.and
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 4
                    local.get 4
                    i32.load offset=4552
                    i32.const 32
                    i32.add
                    i32.store offset=4552
                    br 1 (;@7;)
                  end
                  local.get 4
                  local.get 4
                  i32.load offset=4552
                  i32.const 36
                  i32.add
                  i32.store offset=4552
                  local.get 4
                  i32.const 1
                  i32.store8 offset=4551
                end
                local.get 4
                i32.const 0
                i32.store offset=4572
              end
              br 1 (;@4;)
            end
            block  ;; label = @5
              block  ;; label = @6
                local.get 4
                i32.load offset=4524
                i32.const 65535
                i32.le_u
                i32.const 1
                i32.and
                i32.eqz
                br_if 0 (;@6;)
                local.get 4
                local.get 4
                i32.load offset=6640
                local.get 4
                i32.const 4576
                i32.add
                local.get 0
                i32.load
                local.get 4
                i32.load offset=4536
                call 59
                i32.store offset=4512
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 4
                    i32.load offset=4512
                    i32.const 0
                    i32.ge_s
                    i32.const 1
                    i32.and
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 0
                    i32.const 52
                    i32.add
                    local.get 4
                    i32.load offset=4536
                    i32.add
                    i32.const 1
                    i32.store8
                    local.get 0
                    i32.load
                    local.get 4
                    i32.load offset=4536
                    i32.const 2
                    i32.shl
                    i32.add
                    local.get 4
                    i32.load offset=4512
                    i32.store
                    br 1 (;@7;)
                  end
                  local.get 0
                  i32.const 52
                  i32.add
                  local.get 4
                  i32.load offset=4536
                  i32.add
                  i32.const 2
                  i32.store8
                  local.get 4
                  local.get 4
                  i32.load offset=4552
                  i32.const 32
                  i32.add
                  i32.store offset=4552
                end
                br 1 (;@5;)
              end
              local.get 0
              i32.const 52
              i32.add
              local.get 4
              i32.load offset=4536
              i32.add
              i32.const 3
              i32.store8
              local.get 4
              local.get 4
              i32.load offset=4552
              i32.const 36
              i32.add
              i32.store offset=4552
              local.get 4
              i32.const 1
              i32.store8 offset=4551
            end
          end
          local.get 4
          local.get 4
          i32.load offset=4532
          i32.store offset=4536
          br 0 (;@3;)
        end
      end
      local.get 4
      local.get 4
      i32.load offset=4540
      local.get 4
      i32.load offset=4544
      i32.sub
      i32.const 5
      i32.shr_s
      i32.store offset=4508
      local.get 4
      local.get 4
      i32.load offset=4508
      i32.const 31
      i32.add
      i32.const 5
      i32.shr_s
      i32.store offset=4504
      local.get 4
      local.get 4
      i32.load offset=6632
      local.get 4
      i32.load offset=4504
      i32.add
      local.get 4
      i32.load offset=4552
      i32.add
      local.get 4
      i32.load offset=4508
      i32.add
      i32.const 1
      i32.add
      i32.store offset=4500
      local.get 0
      local.get 4
      i32.load offset=4500
      i32.const 1
      i32.shl
      call 11
      i32.store offset=48
      block  ;; label = @2
        local.get 0
        i32.load offset=48
        i32.const 0
        i32.eq
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        i32.load offset=6636
        i32.const 7
        i32.store
        local.get 4
        i32.const 0
        i32.store offset=6652
        br 1 (;@1;)
      end
      local.get 0
      i32.load offset=48
      local.get 4
      i32.const 4576
      i32.add
      local.get 4
      i32.load offset=6632
      i32.const 1
      i32.shl
      call 9
      drop
      block  ;; label = @2
        local.get 4
        i32.load offset=6640
        local.get 4
        i32.load offset=4500
        i32.const 32
        call 50
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 4
        i32.load offset=6636
        i32.const 7
        i32.store
        local.get 4
        i32.const 0
        i32.store offset=6652
        br 1 (;@1;)
      end
      local.get 4
      i32.const 4472
      i32.add
      call 43
      local.set 1
      block  ;; label = @2
        block  ;; label = @3
          local.get 4
          i32.load8_u offset=4551
          i32.const 1
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          local.get 4
          i32.load offset=4500
          i32.const 36
          call 50
          local.set 2
          block  ;; label = @4
            local.get 2
            i32.const 1
            i32.and
            br_if 0 (;@4;)
            local.get 4
            i32.load offset=6636
            i32.const 7
            i32.store
            local.get 4
            i32.const 0
            i32.store offset=6652
            local.get 4
            i32.const 1
            i32.store offset=4460
            br 2 (;@2;)
          end
        end
        local.get 4
        i32.const 0
        i32.store offset=92
        local.get 4
        local.get 0
        i32.load offset=8
        i32.store offset=4572
        local.get 4
        local.get 4
        i32.load offset=6632
        local.get 4
        i32.load offset=4504
        i32.add
        i32.store offset=88
        local.get 4
        local.get 4
        i32.load offset=88
        i32.store offset=84
        local.get 4
        local.get 4
        i32.load offset=4544
        i32.store offset=80
        block  ;; label = @3
          loop  ;; label = @4
            local.get 4
            i32.load offset=80
            local.get 4
            i32.load offset=4540
            i32.lt_s
            i32.const 1
            i32.and
            i32.eqz
            br_if 1 (;@3;)
            local.get 4
            local.get 0
            i32.const 52
            i32.add
            local.get 4
            i32.load offset=80
            i32.add
            i32.load8_u
            i32.store8 offset=75
            block  ;; label = @5
              local.get 4
              i32.load8_u offset=75
              i32.const 255
              i32.and
              br_if 0 (;@5;)
              local.get 4
              i32.load offset=4572
              i32.const 0
              i32.lt_s
              i32.const 1
              i32.and
              i32.eqz
              br_if 0 (;@5;)
              local.get 4
              i32.const 2
              i32.const 3
              local.get 0
              i32.load offset=24
              i32.const 65535
              i32.le_s
              i32.const 1
              i32.and
              select
              i32.store8 offset=75
              local.get 4
              i32.const 0
              i32.store offset=4572
            end
            block  ;; label = @5
              block  ;; label = @6
                local.get 4
                i32.load8_u offset=75
                i32.const 255
                i32.and
                br_if 0 (;@6;)
                local.get 4
                local.get 0
                i32.load offset=8
                i32.store offset=76
                br 1 (;@5;)
              end
              block  ;; label = @6
                block  ;; label = @7
                  local.get 4
                  i32.load8_u offset=75
                  i32.const 255
                  i32.and
                  i32.const 1
                  i32.eq
                  i32.const 1
                  i32.and
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 4
                  local.get 0
                  i32.load
                  local.get 4
                  i32.load offset=80
                  i32.const 2
                  i32.shl
                  i32.add
                  i32.load
                  i32.store offset=76
                  br 1 (;@6;)
                end
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 4
                    i32.load8_u offset=75
                    i32.const 255
                    i32.and
                    i32.const 2
                    i32.eq
                    i32.const 1
                    i32.and
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 4
                    i32.load offset=6640
                    local.get 0
                    i32.load offset=48
                    local.get 0
                    i32.load
                    local.get 4
                    i32.load offset=80
                    call 59
                    local.set 2
                    local.get 4
                    local.get 2
                    i32.store offset=68
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 4
                        i32.load offset=68
                        i32.const 0
                        i32.ge_s
                        i32.const 1
                        i32.and
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 4
                        local.get 4
                        i32.load offset=68
                        i32.store offset=76
                        br 1 (;@9;)
                      end
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 4
                          i32.load offset=84
                          local.get 4
                          i32.load offset=88
                          i32.eq
                          i32.const 1
                          i32.and
                          i32.eqz
                          br_if 0 (;@11;)
                          local.get 4
                          i32.const 0
                          i32.store offset=68
                          br 1 (;@10;)
                        end
                        local.get 0
                        i32.load offset=48
                        local.get 4
                        i32.load offset=84
                        local.get 0
                        i32.load
                        local.get 4
                        i32.load offset=80
                        call 60
                        local.set 2
                        local.get 4
                        local.get 2
                        i32.store offset=68
                      end
                      local.get 4
                      local.get 4
                      i32.load offset=84
                      local.get 4
                      i32.load offset=68
                      i32.sub
                      i32.store offset=76
                      local.get 4
                      local.get 4
                      i32.load offset=84
                      i32.store offset=64
                      block  ;; label = @10
                        loop  ;; label = @11
                          local.get 4
                          i32.load offset=68
                          i32.const 32
                          i32.lt_s
                          i32.const 1
                          i32.and
                          i32.eqz
                          br_if 1 (;@10;)
                          local.get 0
                          i32.load
                          local.set 2
                          local.get 4
                          i32.load offset=80
                          local.set 3
                          local.get 4
                          local.get 4
                          i32.load offset=68
                          local.tee 5
                          i32.const 1
                          i32.add
                          i32.store offset=68
                          local.get 2
                          local.get 3
                          local.get 5
                          i32.add
                          i32.const 2
                          i32.shl
                          i32.add
                          i32.load
                          local.set 2
                          local.get 0
                          i32.load offset=48
                          local.set 3
                          local.get 4
                          local.get 4
                          i32.load offset=84
                          local.tee 5
                          i32.const 1
                          i32.add
                          i32.store offset=84
                          local.get 3
                          local.get 5
                          i32.const 1
                          i32.shl
                          i32.add
                          local.get 2
                          i32.store16
                          br 0 (;@11;)
                        end
                      end
                      local.get 4
                      i32.load offset=6640
                      local.get 0
                      i32.load offset=48
                      local.get 4
                      i32.load offset=88
                      local.get 4
                      i32.load offset=64
                      local.get 4
                      i32.load offset=84
                      call 58
                      block  ;; label = @10
                        local.get 4
                        i32.load8_u offset=4551
                        i32.const 1
                        i32.and
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 1
                        local.get 0
                        i32.load offset=48
                        local.get 4
                        i32.load offset=88
                        local.get 4
                        i32.load offset=64
                        local.get 4
                        i32.load offset=84
                        call 58
                      end
                    end
                    br 1 (;@7;)
                  end
                  local.get 4
                  local.get 4
                  i32.load offset=80
                  i32.store offset=60
                  local.get 4
                  local.get 4
                  i32.load offset=80
                  i32.const 32
                  i32.add
                  i32.store offset=56
                  local.get 4
                  local.get 4
                  i32.load offset=84
                  i32.store offset=52
                  loop  ;; label = @8
                    local.get 4
                    local.get 4
                    i32.load offset=52
                    i32.const 1
                    i32.add
                    i32.store offset=52
                    local.get 0
                    i32.load
                    local.set 2
                    local.get 4
                    local.get 4
                    i32.load offset=60
                    local.tee 3
                    i32.const 1
                    i32.add
                    i32.store offset=60
                    local.get 4
                    local.get 2
                    local.get 3
                    i32.const 2
                    i32.shl
                    i32.add
                    i32.load
                    i32.store offset=48
                    local.get 4
                    local.get 4
                    i32.load offset=48
                    i32.const 196608
                    i32.and
                    i32.const 2
                    i32.shr_u
                    i32.store offset=44
                    local.get 4
                    i32.load offset=48
                    local.set 2
                    local.get 0
                    i32.load offset=48
                    local.set 3
                    local.get 4
                    local.get 4
                    i32.load offset=52
                    local.tee 5
                    i32.const 1
                    i32.add
                    i32.store offset=52
                    local.get 3
                    local.get 5
                    i32.const 1
                    i32.shl
                    i32.add
                    local.get 2
                    i32.store16
                    local.get 0
                    i32.load
                    local.set 2
                    local.get 4
                    local.get 4
                    i32.load offset=60
                    local.tee 3
                    i32.const 1
                    i32.add
                    i32.store offset=60
                    local.get 4
                    local.get 2
                    local.get 3
                    i32.const 2
                    i32.shl
                    i32.add
                    i32.load
                    i32.store offset=48
                    local.get 4
                    local.get 4
                    i32.load offset=44
                    local.get 4
                    i32.load offset=48
                    i32.const 196608
                    i32.and
                    i32.const 4
                    i32.shr_u
                    i32.or
                    i32.store offset=44
                    local.get 4
                    i32.load offset=48
                    local.set 2
                    local.get 0
                    i32.load offset=48
                    local.set 3
                    local.get 4
                    local.get 4
                    i32.load offset=52
                    local.tee 5
                    i32.const 1
                    i32.add
                    i32.store offset=52
                    local.get 3
                    local.get 5
                    i32.const 1
                    i32.shl
                    i32.add
                    local.get 2
                    i32.store16
                    local.get 0
                    i32.load
                    local.set 2
                    local.get 4
                    local.get 4
                    i32.load offset=60
                    local.tee 3
                    i32.const 1
                    i32.add
                    i32.store offset=60
                    local.get 4
                    local.get 2
                    local.get 3
                    i32.const 2
                    i32.shl
                    i32.add
                    i32.load
                    i32.store offset=48
                    local.get 4
                    local.get 4
                    i32.load offset=44
                    local.get 4
                    i32.load offset=48
                    i32.const 196608
                    i32.and
                    i32.const 6
                    i32.shr_u
                    i32.or
                    i32.store offset=44
                    local.get 4
                    i32.load offset=48
                    local.set 2
                    local.get 0
                    i32.load offset=48
                    local.set 3
                    local.get 4
                    local.get 4
                    i32.load offset=52
                    local.tee 5
                    i32.const 1
                    i32.add
                    i32.store offset=52
                    local.get 3
                    local.get 5
                    i32.const 1
                    i32.shl
                    i32.add
                    local.get 2
                    i32.store16
                    local.get 0
                    i32.load
                    local.set 2
                    local.get 4
                    local.get 4
                    i32.load offset=60
                    local.tee 3
                    i32.const 1
                    i32.add
                    i32.store offset=60
                    local.get 4
                    local.get 2
                    local.get 3
                    i32.const 2
                    i32.shl
                    i32.add
                    i32.load
                    i32.store offset=48
                    local.get 4
                    local.get 4
                    i32.load offset=44
                    local.get 4
                    i32.load offset=48
                    i32.const 196608
                    i32.and
                    i32.const 8
                    i32.shr_u
                    i32.or
                    i32.store offset=44
                    local.get 4
                    i32.load offset=48
                    local.set 2
                    local.get 0
                    i32.load offset=48
                    local.set 3
                    local.get 4
                    local.get 4
                    i32.load offset=52
                    local.tee 5
                    i32.const 1
                    i32.add
                    i32.store offset=52
                    local.get 3
                    local.get 5
                    i32.const 1
                    i32.shl
                    i32.add
                    local.get 2
                    i32.store16
                    local.get 0
                    i32.load
                    local.set 2
                    local.get 4
                    local.get 4
                    i32.load offset=60
                    local.tee 3
                    i32.const 1
                    i32.add
                    i32.store offset=60
                    local.get 4
                    local.get 2
                    local.get 3
                    i32.const 2
                    i32.shl
                    i32.add
                    i32.load
                    i32.store offset=48
                    local.get 4
                    local.get 4
                    i32.load offset=44
                    local.get 4
                    i32.load offset=48
                    i32.const 196608
                    i32.and
                    i32.const 10
                    i32.shr_u
                    i32.or
                    i32.store offset=44
                    local.get 4
                    i32.load offset=48
                    local.set 2
                    local.get 0
                    i32.load offset=48
                    local.set 3
                    local.get 4
                    local.get 4
                    i32.load offset=52
                    local.tee 5
                    i32.const 1
                    i32.add
                    i32.store offset=52
                    local.get 3
                    local.get 5
                    i32.const 1
                    i32.shl
                    i32.add
                    local.get 2
                    i32.store16
                    local.get 0
                    i32.load
                    local.set 2
                    local.get 4
                    local.get 4
                    i32.load offset=60
                    local.tee 3
                    i32.const 1
                    i32.add
                    i32.store offset=60
                    local.get 4
                    local.get 2
                    local.get 3
                    i32.const 2
                    i32.shl
                    i32.add
                    i32.load
                    i32.store offset=48
                    local.get 4
                    local.get 4
                    i32.load offset=44
                    local.get 4
                    i32.load offset=48
                    i32.const 196608
                    i32.and
                    i32.const 12
                    i32.shr_u
                    i32.or
                    i32.store offset=44
                    local.get 4
                    i32.load offset=48
                    local.set 2
                    local.get 0
                    i32.load offset=48
                    local.set 3
                    local.get 4
                    local.get 4
                    i32.load offset=52
                    local.tee 5
                    i32.const 1
                    i32.add
                    i32.store offset=52
                    local.get 3
                    local.get 5
                    i32.const 1
                    i32.shl
                    i32.add
                    local.get 2
                    i32.store16
                    local.get 0
                    i32.load
                    local.set 2
                    local.get 4
                    local.get 4
                    i32.load offset=60
                    local.tee 3
                    i32.const 1
                    i32.add
                    i32.store offset=60
                    local.get 4
                    local.get 2
                    local.get 3
                    i32.const 2
                    i32.shl
                    i32.add
                    i32.load
                    i32.store offset=48
                    local.get 4
                    local.get 4
                    i32.load offset=44
                    local.get 4
                    i32.load offset=48
                    i32.const 196608
                    i32.and
                    i32.const 14
                    i32.shr_u
                    i32.or
                    i32.store offset=44
                    local.get 4
                    i32.load offset=48
                    local.set 2
                    local.get 0
                    i32.load offset=48
                    local.set 3
                    local.get 4
                    local.get 4
                    i32.load offset=52
                    local.tee 5
                    i32.const 1
                    i32.add
                    i32.store offset=52
                    local.get 3
                    local.get 5
                    i32.const 1
                    i32.shl
                    i32.add
                    local.get 2
                    i32.store16
                    local.get 0
                    i32.load
                    local.set 2
                    local.get 4
                    local.get 4
                    i32.load offset=60
                    local.tee 3
                    i32.const 1
                    i32.add
                    i32.store offset=60
                    local.get 4
                    local.get 2
                    local.get 3
                    i32.const 2
                    i32.shl
                    i32.add
                    i32.load
                    i32.store offset=48
                    local.get 4
                    local.get 4
                    i32.load offset=44
                    local.get 4
                    i32.load offset=48
                    i32.const 196608
                    i32.and
                    i32.const 16
                    i32.shr_u
                    i32.or
                    i32.store offset=44
                    local.get 4
                    i32.load offset=48
                    local.set 2
                    local.get 0
                    i32.load offset=48
                    local.set 3
                    local.get 4
                    local.get 4
                    i32.load offset=52
                    local.tee 5
                    i32.const 1
                    i32.add
                    i32.store offset=52
                    local.get 3
                    local.get 5
                    i32.const 1
                    i32.shl
                    i32.add
                    local.get 2
                    i32.store16
                    local.get 0
                    i32.load offset=48
                    local.get 4
                    i32.load offset=52
                    i32.const 9
                    i32.sub
                    i32.const 1
                    i32.shl
                    i32.add
                    local.get 4
                    i32.load offset=44
                    i32.store16
                    local.get 4
                    i32.load offset=60
                    local.get 4
                    i32.load offset=56
                    i32.lt_s
                    i32.const 1
                    i32.and
                    br_if 0 (;@8;)
                  end
                  local.get 1
                  local.get 0
                  i32.load offset=48
                  local.get 0
                  i32.load offset=48
                  local.get 4
                  i32.load offset=84
                  call 61
                  local.set 2
                  local.get 4
                  local.get 2
                  i32.store offset=40
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 4
                      i32.load offset=40
                      i32.const 0
                      i32.ge_s
                      i32.const 1
                      i32.and
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 4
                      local.get 4
                      i32.load offset=40
                      i32.const 32768
                      i32.or
                      i32.store offset=76
                      br 1 (;@8;)
                    end
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 4
                        i32.load offset=84
                        local.get 4
                        i32.load offset=88
                        i32.eq
                        i32.const 1
                        i32.and
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 4
                        i32.const 0
                        i32.store offset=40
                        br 1 (;@9;)
                      end
                      local.get 0
                      i32.load offset=48
                      local.get 4
                      i32.load offset=84
                      local.get 0
                      i32.load offset=48
                      local.get 4
                      i32.load offset=84
                      i32.const 36
                      call 62
                      local.set 2
                      local.get 4
                      local.get 2
                      i32.store offset=40
                    end
                    local.get 4
                    local.get 4
                    i32.load offset=84
                    local.get 4
                    i32.load offset=40
                    i32.sub
                    i32.const 32768
                    i32.or
                    i32.store offset=76
                    local.get 4
                    local.get 4
                    i32.load offset=84
                    i32.store offset=36
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 4
                        i32.load offset=40
                        i32.const 0
                        i32.gt_s
                        i32.const 1
                        i32.and
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 4
                        local.get 4
                        i32.load offset=84
                        i32.store offset=32
                        block  ;; label = @11
                          loop  ;; label = @12
                            local.get 4
                            i32.load offset=40
                            i32.const 36
                            i32.lt_s
                            i32.const 1
                            i32.and
                            i32.eqz
                            br_if 1 (;@11;)
                            local.get 0
                            i32.load offset=48
                            local.set 2
                            local.get 4
                            i32.load offset=32
                            local.set 3
                            local.get 4
                            local.get 4
                            i32.load offset=40
                            local.tee 5
                            i32.const 1
                            i32.add
                            i32.store offset=40
                            local.get 2
                            local.get 3
                            local.get 5
                            i32.add
                            i32.const 1
                            i32.shl
                            i32.add
                            i32.load16_u
                            local.set 2
                            local.get 0
                            i32.load offset=48
                            local.set 3
                            local.get 4
                            local.get 4
                            i32.load offset=84
                            local.tee 5
                            i32.const 1
                            i32.add
                            i32.store offset=84
                            local.get 3
                            local.get 5
                            i32.const 1
                            i32.shl
                            i32.add
                            local.get 2
                            i32.store16
                            br 0 (;@12;)
                          end
                        end
                        br 1 (;@9;)
                      end
                      local.get 4
                      local.get 4
                      i32.load offset=84
                      i32.const 36
                      i32.add
                      i32.store offset=84
                    end
                    local.get 4
                    i32.load offset=6640
                    local.get 0
                    i32.load offset=48
                    local.get 4
                    i32.load offset=88
                    local.get 4
                    i32.load offset=36
                    local.get 4
                    i32.load offset=84
                    call 58
                    block  ;; label = @9
                      local.get 4
                      i32.load8_u offset=4551
                      i32.const 1
                      i32.and
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 1
                      local.get 0
                      i32.load offset=48
                      local.get 4
                      i32.load offset=88
                      local.get 4
                      i32.load offset=36
                      local.get 4
                      i32.load offset=84
                      call 58
                    end
                  end
                end
              end
            end
            block  ;; label = @5
              local.get 0
              i32.load offset=8
              i32.const 0
              i32.lt_s
              i32.const 1
              i32.and
              i32.eqz
              br_if 0 (;@5;)
              local.get 4
              i32.load offset=4572
              i32.const 0
              i32.ge_s
              i32.const 1
              i32.and
              i32.eqz
              br_if 0 (;@5;)
              local.get 0
              local.get 4
              i32.load offset=76
              i32.store offset=8
            end
            local.get 4
            i32.load offset=76
            local.set 2
            local.get 4
            local.get 4
            i32.load offset=92
            local.tee 3
            i32.const 1
            i32.add
            i32.store offset=92
            local.get 4
            i32.const 96
            i32.add
            local.get 3
            i32.const 1
            i32.shl
            i32.add
            local.get 2
            i32.store16
            local.get 4
            local.get 4
            i32.load offset=80
            i32.const 32
            i32.add
            i32.store offset=80
            br 0 (;@4;)
          end
        end
        block  ;; label = @3
          local.get 0
          i32.load offset=8
          i32.const 0
          i32.lt_s
          i32.const 1
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          i32.const 32767
          i32.store offset=8
        end
        block  ;; label = @3
          local.get 4
          i32.load offset=84
          i32.const 32799
          i32.ge_s
          i32.const 1
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          i32.load offset=6636
          i32.const 8
          i32.store
          local.get 4
          i32.const 0
          i32.store offset=6652
          local.get 4
          i32.const 1
          i32.store offset=4460
          br 1 (;@2;)
        end
        local.get 4
        i32.const 32
        i32.store offset=28
        local.get 4
        local.get 4
        i32.load offset=6632
        i32.store offset=24
        local.get 4
        i32.const 0
        i32.store offset=20
        block  ;; label = @3
          loop  ;; label = @4
            local.get 4
            i32.load offset=20
            local.get 4
            i32.load offset=92
            i32.lt_s
            i32.const 1
            i32.and
            i32.eqz
            br_if 1 (;@3;)
            block  ;; label = @5
              block  ;; label = @6
                local.get 4
                i32.load offset=92
                local.get 4
                i32.load offset=20
                i32.sub
                local.get 4
                i32.load offset=28
                i32.ge_s
                i32.const 1
                i32.and
                i32.eqz
                br_if 0 (;@6;)
                local.get 4
                i32.load offset=6640
                local.get 0
                i32.load offset=48
                local.get 4
                i32.const 96
                i32.add
                local.get 4
                i32.load offset=20
                call 61
                local.set 2
                local.get 4
                local.get 2
                i32.store offset=16
                br 1 (;@5;)
              end
              local.get 4
              local.get 4
              i32.load offset=92
              local.get 4
              i32.load offset=20
              i32.sub
              i32.store offset=28
              local.get 0
              i32.load offset=48
              local.get 4
              i32.load offset=88
              local.get 4
              i32.load offset=84
              local.get 4
              i32.const 96
              i32.add
              local.get 4
              i32.load offset=20
              local.get 4
              i32.load offset=28
              call 63
              local.set 2
              local.get 4
              local.get 2
              i32.store offset=16
            end
            block  ;; label = @5
              block  ;; label = @6
                local.get 4
                i32.load offset=16
                i32.const 0
                i32.ge_s
                i32.const 1
                i32.and
                i32.eqz
                br_if 0 (;@6;)
                local.get 4
                local.get 4
                i32.load offset=16
                i32.store offset=12
                br 1 (;@5;)
              end
              block  ;; label = @6
                block  ;; label = @7
                  local.get 4
                  i32.load offset=84
                  local.get 4
                  i32.load offset=88
                  i32.eq
                  i32.const 1
                  i32.and
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 4
                  i32.const 0
                  i32.store offset=16
                  br 1 (;@6;)
                end
                local.get 0
                i32.load offset=48
                local.get 4
                i32.load offset=84
                local.get 4
                i32.const 96
                i32.add
                local.get 4
                i32.load offset=20
                local.get 4
                i32.load offset=28
                call 62
                local.set 2
                local.get 4
                local.get 2
                i32.store offset=16
              end
              local.get 4
              local.get 4
              i32.load offset=84
              local.get 4
              i32.load offset=16
              i32.sub
              i32.store offset=12
              local.get 4
              local.get 4
              i32.load offset=84
              i32.store offset=8
              block  ;; label = @6
                loop  ;; label = @7
                  local.get 4
                  i32.load offset=16
                  local.get 4
                  i32.load offset=28
                  i32.lt_s
                  i32.const 1
                  i32.and
                  i32.eqz
                  br_if 1 (;@6;)
                  local.get 4
                  i32.load offset=20
                  local.set 2
                  local.get 4
                  local.get 4
                  i32.load offset=16
                  local.tee 3
                  i32.const 1
                  i32.add
                  i32.store offset=16
                  local.get 4
                  i32.const 96
                  i32.add
                  local.get 2
                  local.get 3
                  i32.add
                  i32.const 1
                  i32.shl
                  i32.add
                  i32.load16_u
                  local.set 2
                  local.get 0
                  i32.load offset=48
                  local.set 3
                  local.get 4
                  local.get 4
                  i32.load offset=84
                  local.tee 5
                  i32.const 1
                  i32.add
                  i32.store offset=84
                  local.get 3
                  local.get 5
                  i32.const 1
                  i32.shl
                  i32.add
                  local.get 2
                  i32.store16
                  br 0 (;@7;)
                end
              end
              local.get 4
              i32.load offset=6640
              local.get 0
              i32.load offset=48
              local.get 4
              i32.load offset=88
              local.get 4
              i32.load offset=8
              local.get 4
              i32.load offset=84
              call 58
            end
            local.get 4
            i32.load offset=12
            local.set 2
            local.get 0
            i32.load offset=48
            local.set 3
            local.get 4
            local.get 4
            i32.load offset=24
            local.tee 5
            i32.const 1
            i32.add
            i32.store offset=24
            local.get 3
            local.get 5
            i32.const 1
            i32.shl
            i32.add
            local.get 2
            i32.store16
            local.get 4
            local.get 4
            i32.load offset=20
            local.get 4
            i32.load offset=28
            i32.add
            i32.store offset=20
            br 0 (;@4;)
          end
        end
        local.get 4
        local.get 4
        i32.load offset=84
        i32.store offset=6652
        local.get 4
        i32.const 1
        i32.store offset=4460
      end
      local.get 1
      call 46
      drop
    end
    local.get 4
    i32.load offset=6652
    local.set 0
    local.get 4
    i32.const 6656
    i32.add
    global.set 0
    local.get 0)
  (func (;46;) (type 1) (param i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    local.get 0
    i32.store offset=12
    local.get 1
    i32.load offset=12
    local.tee 0
    i32.load
    call 12
    local.get 1
    i32.const 16
    i32.add
    global.set 0
    local.get 0)
  (func (;47;) (type 4) (param i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    local.get 0
    i32.store offset=12
    local.get 3
    local.get 1
    i32.store offset=8
    local.get 3
    local.get 2
    i32.store offset=4
    local.get 3
    local.get 3
    i32.load offset=12
    local.get 3
    i32.load offset=8
    i32.const 2
    i32.shl
    i32.add
    i32.store
    loop  ;; label = @1
      i32.const 0
      local.set 0
      block  ;; label = @2
        local.get 3
        i32.load offset=12
        local.get 3
        i32.load
        i32.lt_u
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.load offset=12
        i32.load
        local.get 3
        i32.load offset=4
        i32.eq
        local.set 0
      end
      block  ;; label = @2
        local.get 0
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        local.get 3
        i32.load offset=12
        i32.const 4
        i32.add
        i32.store offset=12
        br 1 (;@1;)
      end
    end
    local.get 3
    i32.load offset=12
    local.get 3
    i32.load
    i32.eq
    i32.const 1
    i32.and)
  (func (;48;) (type 6) (param i32 i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 4
    local.get 0
    i32.store offset=24
    local.get 4
    local.get 1
    i32.store offset=20
    local.get 4
    local.get 2
    i32.store offset=16
    local.get 4
    local.get 3
    i32.store offset=12
    block  ;; label = @1
      block  ;; label = @2
        local.get 4
        i32.load offset=24
        local.tee 0
        i32.load offset=4
        i32.const 0
        i32.ge_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.const 136
        i32.add
        local.get 0
        i32.load offset=4
        i32.const 2
        i32.shl
        i32.add
        i32.load
        local.get 4
        i32.load offset=12
        i32.eq
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.const 264
        i32.add
        local.get 0
        i32.load offset=4
        i32.const 2
        i32.shl
        i32.add
        local.tee 1
        local.get 1
        i32.load
        local.get 4
        i32.load offset=16
        i32.add
        i32.store
        local.get 4
        local.get 0
        i32.const 8
        i32.add
        local.get 0
        i32.load offset=4
        i32.const 2
        i32.shl
        i32.add
        i32.load
        i32.store offset=28
        br 1 (;@1;)
      end
      local.get 4
      i32.const 0
      i32.store offset=8
      block  ;; label = @2
        loop  ;; label = @3
          local.get 4
          i32.load offset=8
          local.get 0
          i32.load
          i32.lt_s
          i32.const 1
          i32.and
          i32.eqz
          br_if 1 (;@2;)
          block  ;; label = @4
            local.get 0
            i32.const 136
            i32.add
            local.get 4
            i32.load offset=8
            i32.const 2
            i32.shl
            i32.add
            i32.load
            local.get 4
            i32.load offset=12
            i32.eq
            i32.const 1
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            local.get 4
            i32.load offset=8
            i32.store offset=4
            local.get 0
            i32.const 264
            i32.add
            local.get 4
            i32.load offset=8
            i32.const 2
            i32.shl
            i32.add
            local.tee 1
            local.get 1
            i32.load
            local.get 4
            i32.load offset=16
            i32.add
            i32.store
            local.get 4
            local.get 0
            i32.const 8
            i32.add
            local.get 4
            i32.load offset=8
            i32.const 2
            i32.shl
            i32.add
            i32.load
            i32.store offset=28
            br 3 (;@1;)
          end
          local.get 4
          local.get 4
          i32.load offset=8
          i32.const 1
          i32.add
          i32.store offset=8
          br 0 (;@3;)
        end
      end
      block  ;; label = @2
        local.get 0
        i32.load
        i32.const 32
        i32.eq
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        i32.const -2
        i32.store offset=28
        br 1 (;@1;)
      end
      local.get 0
      local.get 0
      i32.load
      i32.store offset=4
      local.get 0
      i32.const 8
      i32.add
      local.get 0
      i32.load
      i32.const 2
      i32.shl
      i32.add
      local.get 4
      i32.load offset=20
      i32.store
      local.get 0
      i32.const 136
      i32.add
      local.get 0
      i32.load
      i32.const 2
      i32.shl
      i32.add
      local.get 4
      i32.load offset=12
      i32.store
      local.get 4
      i32.load offset=16
      local.set 1
      local.get 0
      local.get 0
      i32.load
      local.tee 2
      i32.const 1
      i32.add
      i32.store
      local.get 0
      i32.const 264
      i32.add
      local.get 2
      i32.const 2
      i32.shl
      i32.add
      local.get 1
      i32.store
      local.get 4
      i32.const -1
      i32.store offset=28
    end
    local.get 4
    i32.load offset=28)
  (func (;49;) (type 7) (param i32 i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 4
    local.get 0
    i32.store offset=28
    local.get 4
    local.get 1
    i32.store offset=24
    local.get 4
    local.get 2
    i32.store offset=20
    local.get 4
    local.get 3
    i32.store offset=16
    local.get 4
    i32.load offset=28
    local.set 0
    local.get 4
    i32.const -1
    i32.store offset=12
    local.get 4
    i32.const 69632
    i32.store offset=8
    local.get 4
    i32.const 0
    i32.store offset=4
    block  ;; label = @1
      loop  ;; label = @2
        local.get 4
        i32.load offset=4
        local.get 0
        i32.load
        i32.lt_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        block  ;; label = @3
          local.get 0
          i32.const 264
          i32.add
          local.get 4
          i32.load offset=4
          i32.const 2
          i32.shl
          i32.add
          i32.load
          local.get 4
          i32.load offset=8
          i32.lt_s
          i32.const 1
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          local.get 4
          i32.load offset=4
          i32.store offset=12
          local.get 4
          local.get 0
          i32.const 264
          i32.add
          local.get 4
          i32.load offset=4
          i32.const 2
          i32.shl
          i32.add
          i32.load
          i32.store offset=8
        end
        local.get 4
        local.get 4
        i32.load offset=4
        i32.const 1
        i32.add
        i32.store offset=4
        br 0 (;@2;)
      end
    end
    local.get 0
    local.get 4
    i32.load offset=12
    i32.store offset=4
    local.get 0
    i32.const 8
    i32.add
    local.get 4
    i32.load offset=12
    i32.const 2
    i32.shl
    i32.add
    local.get 4
    i32.load offset=24
    i32.store
    local.get 0
    i32.const 136
    i32.add
    local.get 4
    i32.load offset=12
    i32.const 2
    i32.shl
    i32.add
    local.get 4
    i32.load offset=16
    i32.store
    local.get 0
    i32.const 264
    i32.add
    local.get 4
    i32.load offset=12
    i32.const 2
    i32.shl
    i32.add
    local.get 4
    i32.load offset=20
    i32.store)
  (func (;50;) (type 4) (param i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    local.get 0
    i32.store offset=24
    local.get 3
    local.get 1
    i32.store offset=20
    local.get 3
    local.get 2
    i32.store offset=16
    local.get 3
    i32.load offset=24
    local.set 0
    local.get 3
    local.get 3
    i32.load offset=20
    local.get 3
    i32.load offset=16
    i32.sub
    i32.const 1
    i32.add
    i32.store offset=12
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        i32.load offset=12
        i32.const 4095
        i32.le_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.const 6007
        i32.store offset=8
        local.get 0
        i32.const 12
        i32.store offset=12
        local.get 0
        i32.const 4095
        i32.store offset=16
        br 1 (;@1;)
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 3
          i32.load offset=12
          i32.const 32767
          i32.le_s
          i32.const 1
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          i32.const 50021
          i32.store offset=8
          local.get 0
          i32.const 15
          i32.store offset=12
          local.get 0
          i32.const 32767
          i32.store offset=16
          br 1 (;@2;)
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 3
            i32.load offset=12
            i32.const 131071
            i32.le_s
            i32.const 1
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 3
            i32.const 200003
            i32.store offset=8
            local.get 0
            i32.const 17
            i32.store offset=12
            local.get 0
            i32.const 131071
            i32.store offset=16
            br 1 (;@3;)
          end
          local.get 3
          i32.const 1500007
          i32.store offset=8
          local.get 0
          i32.const 21
          i32.store offset=12
          local.get 0
          i32.const 2097151
          i32.store offset=16
        end
      end
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        i32.load offset=8
        local.get 0
        i32.load offset=4
        i32.gt_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.load
        call 12
        local.get 0
        local.get 3
        i32.load offset=8
        i32.const 2
        i32.shl
        call 11
        i32.store
        block  ;; label = @3
          local.get 0
          i32.load
          i32.const 0
          i32.eq
          i32.const 1
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          i32.const 0
          i32.const 1
          i32.and
          i32.store8 offset=31
          br 2 (;@1;)
        end
        local.get 0
        local.get 3
        i32.load offset=8
        i32.store offset=4
      end
      local.get 0
      local.get 3
      i32.load offset=8
      i32.store offset=8
      local.get 0
      i32.load
      i32.const 0
      local.get 0
      i32.load offset=8
      i32.const 2
      i32.shl
      call 10
      drop
      local.get 0
      local.get 3
      i32.load offset=16
      i32.store offset=20
      local.get 3
      i32.const 1
      i32.const 1
      i32.and
      i32.store8 offset=31
    end
    local.get 3
    i32.load8_u offset=31
    local.set 0
    local.get 3
    i32.const 32
    i32.add
    global.set 0
    local.get 0
    i32.const 1
    i32.and)
  (func (;51;) (type 7) (param i32 i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 4
    global.set 0
    local.get 4
    local.get 0
    i32.store offset=28
    local.get 4
    local.get 1
    i32.store offset=24
    local.get 4
    i32.const 0
    i32.store offset=20
    local.get 4
    local.get 2
    i32.store offset=16
    local.get 4
    local.get 3
    i32.store offset=12
    local.get 4
    local.get 4
    i32.load offset=16
    local.get 4
    i32.load offset=28
    local.tee 0
    i32.load offset=20
    i32.sub
    i32.store offset=8
    block  ;; label = @1
      block  ;; label = @2
        local.get 4
        i32.load offset=8
        local.get 4
        i32.load offset=20
        i32.ge_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        local.get 4
        i32.load offset=8
        i32.const 1
        i32.add
        i32.store offset=8
        br 1 (;@1;)
      end
      local.get 4
      local.get 4
      i32.load offset=20
      i32.store offset=8
    end
    local.get 4
    local.get 4
    i32.load offset=12
    local.get 0
    i32.load offset=20
    i32.sub
    i32.store offset=4
    block  ;; label = @1
      loop  ;; label = @2
        local.get 4
        i32.load offset=8
        local.get 4
        i32.load offset=4
        i32.le_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 4
        local.get 0
        local.get 4
        i32.load offset=24
        local.get 4
        i32.load offset=8
        call 66
        i32.store
        local.get 0
        local.get 4
        i32.load offset=24
        local.get 4
        i32.load offset=8
        local.get 4
        i32.load
        local.get 4
        i32.load offset=8
        call 72
        local.get 4
        local.get 4
        i32.load offset=8
        i32.const 1
        i32.add
        i32.store offset=8
        br 0 (;@2;)
      end
    end
    local.get 4
    i32.const 32
    i32.add
    global.set 0)
  (func (;52;) (type 4) (param i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    local.get 0
    i32.store offset=24
    local.get 3
    local.get 1
    i32.store offset=20
    local.get 3
    local.get 2
    i32.store offset=16
    local.get 3
    local.get 3
    i32.load offset=24
    local.tee 0
    local.get 3
    i32.load offset=16
    call 73
    i32.store offset=12
    local.get 3
    local.get 0
    local.get 3
    i32.load offset=20
    local.get 3
    i32.load offset=16
    local.get 3
    i32.load offset=12
    call 74
    i32.store offset=8
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        i32.load offset=8
        i32.const 0
        i32.ge_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        local.get 0
        i32.load
        local.get 3
        i32.load offset=8
        i32.const 2
        i32.shl
        i32.add
        i32.load
        local.get 0
        i32.load offset=16
        i32.and
        i32.const 1
        i32.sub
        i32.store offset=28
        br 1 (;@1;)
      end
      local.get 3
      i32.const -1
      i32.store offset=28
    end
    local.get 3
    i32.load offset=28
    local.set 0
    local.get 3
    i32.const 32
    i32.add
    global.set 0
    local.get 0)
  (func (;53;) (type 4) (param i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    local.get 0
    i32.store offset=24
    local.get 3
    local.get 1
    i32.store offset=20
    local.get 3
    local.get 2
    i32.store offset=16
    local.get 3
    i32.const 0
    i32.store offset=12
    block  ;; label = @1
      block  ;; label = @2
        loop  ;; label = @3
          local.get 3
          i32.load offset=12
          local.get 3
          i32.load offset=16
          i32.lt_s
          i32.const 1
          i32.and
          i32.eqz
          br_if 1 (;@2;)
          block  ;; label = @4
            local.get 3
            i32.load offset=20
            local.get 3
            i32.load offset=12
            i32.const 2
            i32.shl
            i32.add
            i32.load
            local.get 3
            i32.load offset=24
            i32.eq
            i32.const 1
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 3
            i32.const 1
            i32.const 1
            i32.and
            i32.store8 offset=31
            br 3 (;@1;)
          end
          local.get 3
          local.get 3
          i32.load offset=12
          i32.const 4
          i32.add
          i32.store offset=12
          br 0 (;@3;)
        end
      end
      local.get 3
      i32.const 0
      i32.const 1
      i32.and
      i32.store8 offset=31
    end
    local.get 3
    i32.load8_u offset=31
    i32.const 1
    i32.and)
  (func (;54;) (type 10) (param i32 i32 i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 5
    local.get 0
    i32.store offset=24
    local.get 5
    local.get 1
    i32.store offset=20
    local.get 5
    local.get 2
    i32.store offset=16
    local.get 5
    local.get 3
    i32.store offset=12
    local.get 5
    local.get 4
    i32.store offset=8
    local.get 5
    local.get 5
    i32.load offset=16
    local.get 5
    i32.load offset=8
    i32.sub
    i32.store offset=16
    local.get 5
    local.get 5
    i32.load offset=20
    i32.store offset=4
    block  ;; label = @1
      block  ;; label = @2
        loop  ;; label = @3
          local.get 5
          i32.load offset=4
          local.get 5
          i32.load offset=16
          i32.le_s
          i32.const 1
          i32.and
          i32.eqz
          br_if 1 (;@2;)
          block  ;; label = @4
            local.get 5
            i32.load offset=24
            local.get 5
            i32.load offset=4
            i32.const 2
            i32.shl
            i32.add
            i32.load
            local.get 5
            i32.load offset=12
            i32.eq
            i32.const 1
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 5
            i32.const 1
            i32.store
            loop  ;; label = @5
              block  ;; label = @6
                local.get 5
                i32.load
                local.get 5
                i32.load offset=8
                i32.eq
                i32.const 1
                i32.and
                i32.eqz
                br_if 0 (;@6;)
                local.get 5
                local.get 5
                i32.load offset=4
                i32.store offset=28
                br 5 (;@1;)
              end
              block  ;; label = @6
                block  ;; label = @7
                  local.get 5
                  i32.load offset=24
                  local.get 5
                  i32.load offset=4
                  local.get 5
                  i32.load
                  i32.add
                  i32.const 2
                  i32.shl
                  i32.add
                  i32.load
                  local.get 5
                  i32.load offset=12
                  i32.ne
                  i32.const 1
                  i32.and
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 5
                  local.get 5
                  i32.load offset=4
                  local.get 5
                  i32.load
                  i32.add
                  i32.store offset=4
                  br 1 (;@6;)
                end
                local.get 5
                local.get 5
                i32.load
                i32.const 1
                i32.add
                i32.store
                br 1 (;@5;)
              end
            end
          end
          local.get 5
          local.get 5
          i32.load offset=4
          i32.const 1
          i32.add
          i32.store offset=4
          br 0 (;@3;)
        end
      end
      local.get 5
      i32.const -1
      i32.store offset=28
    end
    local.get 5
    i32.load offset=28)
  (func (;55;) (type 6) (param i32 i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 4
    local.get 0
    i32.store offset=28
    local.get 4
    local.get 1
    i32.store offset=24
    local.get 4
    local.get 2
    i32.store offset=20
    local.get 4
    local.get 3
    i32.store offset=16
    local.get 4
    local.get 4
    i32.load offset=24
    local.get 4
    i32.load offset=16
    i32.const 1
    i32.sub
    i32.sub
    i32.store offset=12
    local.get 4
    local.get 4
    i32.load offset=24
    i32.store offset=8
    loop  ;; label = @1
      i32.const 0
      local.set 0
      block  ;; label = @2
        local.get 4
        i32.load offset=12
        local.get 4
        i32.load offset=8
        i32.lt_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        i32.load offset=28
        local.get 4
        i32.load offset=8
        i32.const 1
        i32.sub
        i32.const 2
        i32.shl
        i32.add
        i32.load
        local.get 4
        i32.load offset=20
        i32.eq
        local.set 0
      end
      block  ;; label = @2
        local.get 0
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        local.get 4
        i32.load offset=8
        i32.const -1
        i32.add
        i32.store offset=8
        br 1 (;@1;)
      end
    end
    local.get 4
    i32.load offset=24
    local.get 4
    i32.load offset=8
    i32.sub)
  (func (;56;) (type 4) (param i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    local.get 0
    i32.store offset=24
    local.get 3
    local.get 1
    i32.store offset=20
    local.get 3
    local.get 2
    i32.store offset=16
    local.get 3
    i32.const 0
    i32.store offset=12
    local.get 3
    local.get 3
    i32.load offset=24
    local.tee 0
    local.get 3
    i32.load offset=16
    local.get 3
    i32.load offset=12
    call 66
    i32.store offset=8
    local.get 3
    local.get 0
    local.get 3
    i32.load offset=20
    local.get 3
    i32.load offset=16
    local.get 3
    i32.load offset=12
    local.get 3
    i32.load offset=8
    call 75
    i32.store offset=4
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        i32.load offset=4
        i32.const 0
        i32.ge_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        local.get 0
        i32.load
        local.get 3
        i32.load offset=4
        i32.const 2
        i32.shl
        i32.add
        i32.load
        local.get 0
        i32.load offset=16
        i32.and
        i32.const 1
        i32.sub
        i32.store offset=28
        br 1 (;@1;)
      end
      local.get 3
      i32.const -1
      i32.store offset=28
    end
    local.get 3
    i32.load offset=28
    local.set 0
    local.get 3
    i32.const 32
    i32.add
    global.set 0
    local.get 0)
  (func (;57;) (type 6) (param i32 i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 4
    global.set 0
    local.get 4
    local.get 0
    i32.store offset=28
    local.get 4
    local.get 1
    i32.store offset=24
    local.get 4
    local.get 2
    i32.store offset=20
    local.get 4
    i32.const 0
    i32.store offset=16
    local.get 4
    local.get 3
    i32.store offset=12
    local.get 4
    local.get 4
    i32.load offset=12
    i32.const 1
    i32.sub
    i32.store offset=8
    local.get 4
    local.get 4
    i32.load offset=20
    local.get 4
    i32.load offset=16
    i32.const 2
    i32.shl
    i32.add
    i32.store offset=20
    loop  ;; label = @1
      i32.const 0
      local.set 0
      block  ;; label = @2
        local.get 4
        i32.load offset=8
        i32.const 0
        i32.gt_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        i32.load offset=28
        local.get 4
        i32.load offset=24
        local.get 4
        i32.load offset=8
        i32.sub
        i32.const 2
        i32.shl
        i32.add
        local.get 4
        i32.load offset=20
        local.get 4
        i32.load offset=8
        call 76
        i32.const -1
        i32.xor
        local.set 0
      end
      block  ;; label = @2
        local.get 0
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        local.get 4
        i32.load offset=8
        i32.const -1
        i32.add
        i32.store offset=8
        br 1 (;@1;)
      end
    end
    local.get 4
    i32.load offset=8
    local.set 0
    local.get 4
    i32.const 32
    i32.add
    global.set 0
    local.get 0)
  (func (;58;) (type 11) (param i32 i32 i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 5
    global.set 0
    local.get 5
    local.get 0
    i32.store offset=28
    local.get 5
    local.get 1
    i32.store offset=24
    local.get 5
    local.get 2
    i32.store offset=20
    local.get 5
    local.get 3
    i32.store offset=16
    local.get 5
    local.get 4
    i32.store offset=12
    local.get 5
    local.get 5
    i32.load offset=16
    local.get 5
    i32.load offset=28
    local.tee 0
    i32.load offset=20
    i32.sub
    i32.store offset=8
    block  ;; label = @1
      block  ;; label = @2
        local.get 5
        i32.load offset=8
        local.get 5
        i32.load offset=20
        i32.ge_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 5
        local.get 5
        i32.load offset=8
        i32.const 1
        i32.add
        i32.store offset=8
        br 1 (;@1;)
      end
      local.get 5
      local.get 5
      i32.load offset=20
      i32.store offset=8
    end
    local.get 5
    local.get 5
    i32.load offset=12
    local.get 0
    i32.load offset=20
    i32.sub
    i32.store offset=4
    block  ;; label = @1
      loop  ;; label = @2
        local.get 5
        i32.load offset=8
        local.get 5
        i32.load offset=4
        i32.le_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 5
        local.get 0
        local.get 5
        i32.load offset=24
        local.get 5
        i32.load offset=8
        call 64
        i32.store
        local.get 0
        local.get 5
        i32.load offset=24
        local.get 5
        i32.load offset=8
        local.get 5
        i32.load
        local.get 5
        i32.load offset=8
        call 65
        local.get 5
        local.get 5
        i32.load offset=8
        i32.const 1
        i32.add
        i32.store offset=8
        br 0 (;@2;)
      end
    end
    local.get 5
    i32.const 32
    i32.add
    global.set 0)
  (func (;59;) (type 6) (param i32 i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 4
    global.set 0
    local.get 4
    local.get 0
    i32.store offset=24
    local.get 4
    local.get 1
    i32.store offset=20
    local.get 4
    local.get 2
    i32.store offset=16
    local.get 4
    local.get 3
    i32.store offset=12
    local.get 4
    local.get 4
    i32.load offset=24
    local.tee 0
    local.get 4
    i32.load offset=16
    local.get 4
    i32.load offset=12
    call 66
    i32.store offset=8
    local.get 4
    local.get 0
    local.get 4
    i32.load offset=20
    local.get 4
    i32.load offset=16
    local.get 4
    i32.load offset=12
    local.get 4
    i32.load offset=8
    call 67
    i32.store offset=4
    block  ;; label = @1
      block  ;; label = @2
        local.get 4
        i32.load offset=4
        i32.const 0
        i32.ge_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        local.get 0
        i32.load
        local.get 4
        i32.load offset=4
        i32.const 2
        i32.shl
        i32.add
        i32.load
        local.get 0
        i32.load offset=16
        i32.and
        i32.const 1
        i32.sub
        i32.store offset=28
        br 1 (;@1;)
      end
      local.get 4
      i32.const -1
      i32.store offset=28
    end
    local.get 4
    i32.load offset=28
    local.set 0
    local.get 4
    i32.const 32
    i32.add
    global.set 0
    local.get 0)
  (func (;60;) (type 6) (param i32 i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 4
    global.set 0
    local.get 4
    local.get 0
    i32.store offset=28
    local.get 4
    local.get 1
    i32.store offset=24
    local.get 4
    local.get 2
    i32.store offset=20
    local.get 4
    local.get 3
    i32.store offset=16
    local.get 4
    i32.const 32
    i32.store offset=12
    local.get 4
    local.get 4
    i32.load offset=12
    i32.const 1
    i32.sub
    i32.store offset=8
    local.get 4
    local.get 4
    i32.load offset=20
    local.get 4
    i32.load offset=16
    i32.const 2
    i32.shl
    i32.add
    i32.store offset=20
    loop  ;; label = @1
      i32.const 0
      local.set 0
      block  ;; label = @2
        local.get 4
        i32.load offset=8
        i32.const 0
        i32.gt_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        i32.load offset=28
        local.get 4
        i32.load offset=24
        local.get 4
        i32.load offset=8
        i32.sub
        i32.const 1
        i32.shl
        i32.add
        local.get 4
        i32.load offset=20
        local.get 4
        i32.load offset=8
        call 68
        i32.const -1
        i32.xor
        local.set 0
      end
      block  ;; label = @2
        local.get 0
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        local.get 4
        i32.load offset=8
        i32.const -1
        i32.add
        i32.store offset=8
        br 1 (;@1;)
      end
    end
    local.get 4
    i32.load offset=8
    local.set 0
    local.get 4
    i32.const 32
    i32.add
    global.set 0
    local.get 0)
  (func (;61;) (type 6) (param i32 i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 4
    global.set 0
    local.get 4
    local.get 0
    i32.store offset=24
    local.get 4
    local.get 1
    i32.store offset=20
    local.get 4
    local.get 2
    i32.store offset=16
    local.get 4
    local.get 3
    i32.store offset=12
    local.get 4
    local.get 4
    i32.load offset=24
    local.tee 0
    local.get 4
    i32.load offset=16
    local.get 4
    i32.load offset=12
    call 64
    i32.store offset=8
    local.get 4
    local.get 0
    local.get 4
    i32.load offset=20
    local.get 4
    i32.load offset=16
    local.get 4
    i32.load offset=12
    local.get 4
    i32.load offset=8
    call 69
    i32.store offset=4
    block  ;; label = @1
      block  ;; label = @2
        local.get 4
        i32.load offset=4
        i32.const 0
        i32.ge_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        local.get 0
        i32.load
        local.get 4
        i32.load offset=4
        i32.const 2
        i32.shl
        i32.add
        i32.load
        local.get 0
        i32.load offset=16
        i32.and
        i32.const 1
        i32.sub
        i32.store offset=28
        br 1 (;@1;)
      end
      local.get 4
      i32.const -1
      i32.store offset=28
    end
    local.get 4
    i32.load offset=28
    local.set 0
    local.get 4
    i32.const 32
    i32.add
    global.set 0
    local.get 0)
  (func (;62;) (type 10) (param i32 i32 i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 5
    global.set 0
    local.get 5
    local.get 0
    i32.store offset=28
    local.get 5
    local.get 1
    i32.store offset=24
    local.get 5
    local.get 2
    i32.store offset=20
    local.get 5
    local.get 3
    i32.store offset=16
    local.get 5
    local.get 4
    i32.store offset=12
    local.get 5
    local.get 5
    i32.load offset=12
    i32.const 1
    i32.sub
    i32.store offset=8
    local.get 5
    local.get 5
    i32.load offset=20
    local.get 5
    i32.load offset=16
    i32.const 1
    i32.shl
    i32.add
    i32.store offset=20
    loop  ;; label = @1
      i32.const 0
      local.set 0
      block  ;; label = @2
        local.get 5
        i32.load offset=8
        i32.const 0
        i32.gt_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 5
        i32.load offset=28
        local.get 5
        i32.load offset=24
        local.get 5
        i32.load offset=8
        i32.sub
        i32.const 1
        i32.shl
        i32.add
        local.get 5
        i32.load offset=20
        local.get 5
        i32.load offset=8
        call 70
        i32.const -1
        i32.xor
        local.set 0
      end
      block  ;; label = @2
        local.get 0
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 5
        local.get 5
        i32.load offset=8
        i32.const -1
        i32.add
        i32.store offset=8
        br 1 (;@1;)
      end
    end
    local.get 5
    i32.load offset=8
    local.set 0
    local.get 5
    i32.const 32
    i32.add
    global.set 0
    local.get 0)
  (func (;63;) (type 12) (param i32 i32 i32 i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 6
    global.set 0
    local.get 6
    local.get 0
    i32.store offset=24
    local.get 6
    local.get 1
    i32.store offset=20
    local.get 6
    local.get 2
    i32.store offset=16
    local.get 6
    local.get 3
    i32.store offset=12
    local.get 6
    local.get 4
    i32.store offset=8
    local.get 6
    local.get 5
    i32.store offset=4
    local.get 6
    local.get 6
    i32.load offset=16
    local.get 6
    i32.load offset=4
    i32.sub
    i32.store offset=16
    local.get 6
    local.get 6
    i32.load offset=12
    local.get 6
    i32.load offset=8
    i32.const 1
    i32.shl
    i32.add
    i32.store offset=12
    block  ;; label = @1
      block  ;; label = @2
        loop  ;; label = @3
          local.get 6
          i32.load offset=20
          local.get 6
          i32.load offset=16
          i32.le_s
          i32.const 1
          i32.and
          i32.eqz
          br_if 1 (;@2;)
          block  ;; label = @4
            local.get 6
            i32.load offset=24
            local.get 6
            i32.load offset=20
            i32.const 1
            i32.shl
            i32.add
            local.get 6
            i32.load offset=12
            local.get 6
            i32.load offset=4
            call 70
            i32.const 1
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            local.get 6
            i32.load offset=20
            i32.store offset=28
            br 3 (;@1;)
          end
          local.get 6
          local.get 6
          i32.load offset=20
          i32.const 1
          i32.add
          i32.store offset=20
          br 0 (;@3;)
        end
      end
      local.get 6
      i32.const -1
      i32.store offset=28
    end
    local.get 6
    i32.load offset=28
    local.set 0
    local.get 6
    i32.const 32
    i32.add
    global.set 0
    local.get 0)
  (func (;64;) (type 4) (param i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    local.get 0
    i32.store offset=28
    local.get 3
    local.get 1
    i32.store offset=24
    local.get 3
    local.get 2
    i32.store offset=20
    local.get 3
    local.get 3
    i32.load offset=20
    local.get 3
    i32.load offset=28
    i32.load offset=20
    i32.add
    i32.store offset=16
    local.get 3
    i32.load offset=24
    local.set 0
    local.get 3
    local.get 3
    i32.load offset=20
    local.tee 1
    i32.const 1
    i32.add
    i32.store offset=20
    local.get 3
    local.get 0
    local.get 1
    i32.const 1
    i32.shl
    i32.add
    i32.load16_u
    i32.const 65535
    i32.and
    i32.store offset=12
    loop  ;; label = @1
      local.get 3
      i32.load offset=12
      local.set 0
      local.get 3
      i32.load offset=24
      local.set 1
      local.get 3
      local.get 3
      i32.load offset=20
      local.tee 2
      i32.const 1
      i32.add
      i32.store offset=20
      local.get 3
      local.get 0
      i32.const 37
      i32.mul
      local.get 1
      local.get 2
      i32.const 1
      i32.shl
      i32.add
      i32.load16_u
      i32.const 65535
      i32.and
      i32.add
      i32.store offset=12
      local.get 3
      i32.load offset=20
      local.get 3
      i32.load offset=16
      i32.lt_s
      i32.const 1
      i32.and
      br_if 0 (;@1;)
    end
    local.get 3
    i32.load offset=12)
  (func (;65;) (type 11) (param i32 i32 i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 5
    global.set 0
    local.get 5
    local.get 0
    i32.store offset=28
    local.get 5
    local.get 1
    i32.store offset=24
    local.get 5
    local.get 2
    i32.store offset=20
    local.get 5
    local.get 3
    i32.store offset=16
    local.get 5
    local.get 4
    i32.store offset=12
    local.get 5
    local.get 5
    i32.load offset=28
    local.tee 0
    local.get 5
    i32.load offset=24
    local.get 5
    i32.load offset=24
    local.get 5
    i32.load offset=20
    local.get 5
    i32.load offset=16
    call 69
    i32.store offset=8
    block  ;; label = @1
      local.get 5
      i32.load offset=8
      i32.const 0
      i32.lt_s
      i32.const 1
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load
      local.get 5
      i32.load offset=8
      i32.const -1
      i32.xor
      i32.const 2
      i32.shl
      i32.add
      local.get 5
      i32.load offset=16
      local.get 0
      i32.load offset=12
      i32.shl
      local.get 5
      i32.load offset=12
      i32.const 1
      i32.add
      i32.or
      i32.store
    end
    local.get 5
    i32.const 32
    i32.add
    global.set 0)
  (func (;66;) (type 4) (param i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    local.get 0
    i32.store offset=28
    local.get 3
    local.get 1
    i32.store offset=24
    local.get 3
    local.get 2
    i32.store offset=20
    local.get 3
    local.get 3
    i32.load offset=20
    local.get 3
    i32.load offset=28
    i32.load offset=20
    i32.add
    i32.store offset=16
    local.get 3
    i32.load offset=24
    local.set 0
    local.get 3
    local.get 3
    i32.load offset=20
    local.tee 1
    i32.const 1
    i32.add
    i32.store offset=20
    local.get 3
    local.get 0
    local.get 1
    i32.const 2
    i32.shl
    i32.add
    i32.load
    i32.store offset=12
    loop  ;; label = @1
      local.get 3
      i32.load offset=12
      local.set 0
      local.get 3
      i32.load offset=24
      local.set 1
      local.get 3
      local.get 3
      i32.load offset=20
      local.tee 2
      i32.const 1
      i32.add
      i32.store offset=20
      local.get 3
      local.get 0
      i32.const 37
      i32.mul
      local.get 1
      local.get 2
      i32.const 2
      i32.shl
      i32.add
      i32.load
      i32.add
      i32.store offset=12
      local.get 3
      i32.load offset=20
      local.get 3
      i32.load offset=16
      i32.lt_s
      i32.const 1
      i32.and
      br_if 0 (;@1;)
    end
    local.get 3
    i32.load offset=12)
  (func (;67;) (type 10) (param i32 i32 i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 5
    global.set 0
    local.get 5
    local.get 0
    i32.store offset=40
    local.get 5
    local.get 1
    i32.store offset=36
    local.get 5
    local.get 2
    i32.store offset=32
    local.get 5
    local.get 3
    i32.store offset=28
    local.get 5
    local.get 4
    i32.store offset=24
    local.get 5
    local.get 5
    i32.load offset=24
    local.get 5
    i32.load offset=40
    local.tee 0
    i32.load offset=12
    i32.shl
    i32.store offset=20
    local.get 5
    local.get 5
    i32.load offset=24
    local.get 0
    i32.load offset=8
    i32.const 1
    i32.sub
    i32.rem_u
    i32.const 1
    i32.add
    i32.store offset=16
    local.get 5
    local.get 5
    i32.load offset=16
    i32.store offset=12
    block  ;; label = @1
      loop  ;; label = @2
        local.get 5
        local.get 0
        i32.load
        local.get 5
        i32.load offset=12
        i32.const 2
        i32.shl
        i32.add
        i32.load
        i32.store offset=8
        block  ;; label = @3
          local.get 5
          i32.load offset=8
          br_if 0 (;@3;)
          local.get 5
          local.get 5
          i32.load offset=12
          i32.const -1
          i32.xor
          i32.store offset=44
          br 2 (;@1;)
        end
        block  ;; label = @3
          local.get 5
          i32.load offset=8
          local.get 0
          i32.load offset=16
          i32.const -1
          i32.xor
          i32.and
          local.get 5
          i32.load offset=20
          i32.eq
          i32.const 1
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 5
          local.get 5
          i32.load offset=8
          local.get 0
          i32.load offset=16
          i32.and
          i32.const 1
          i32.sub
          i32.store offset=4
          block  ;; label = @4
            local.get 5
            i32.load offset=36
            local.get 5
            i32.load offset=4
            i32.const 1
            i32.shl
            i32.add
            local.get 5
            i32.load offset=32
            local.get 5
            i32.load offset=28
            i32.const 2
            i32.shl
            i32.add
            local.get 0
            i32.load offset=20
            call 68
            i32.const 1
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 5
            local.get 5
            i32.load offset=12
            i32.store offset=44
            br 3 (;@1;)
          end
        end
        local.get 5
        local.get 0
        local.get 5
        i32.load offset=16
        local.get 5
        i32.load offset=12
        call 71
        i32.store offset=12
        br 0 (;@2;)
      end
    end
    local.get 5
    i32.load offset=44
    local.set 0
    local.get 5
    i32.const 48
    i32.add
    global.set 0
    local.get 0)
  (func (;68;) (type 4) (param i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    local.get 0
    i32.store offset=12
    local.get 3
    local.get 1
    i32.store offset=8
    local.get 3
    local.get 2
    i32.store offset=4
    loop  ;; label = @1
      i32.const 0
      local.set 0
      block  ;; label = @2
        local.get 3
        i32.load offset=4
        i32.const 0
        i32.gt_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.load offset=12
        i32.load16_u
        i32.const 65535
        i32.and
        local.get 3
        i32.load offset=8
        i32.load
        i32.eq
        local.set 0
      end
      block  ;; label = @2
        local.get 0
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        local.get 3
        i32.load offset=12
        i32.const 2
        i32.add
        i32.store offset=12
        local.get 3
        local.get 3
        i32.load offset=8
        i32.const 4
        i32.add
        i32.store offset=8
        local.get 3
        local.get 3
        i32.load offset=4
        i32.const -1
        i32.add
        i32.store offset=4
        br 1 (;@1;)
      end
    end
    local.get 3
    i32.load offset=4
    i32.const 0
    i32.eq
    i32.const 1
    i32.and)
  (func (;69;) (type 10) (param i32 i32 i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 5
    global.set 0
    local.get 5
    local.get 0
    i32.store offset=40
    local.get 5
    local.get 1
    i32.store offset=36
    local.get 5
    local.get 2
    i32.store offset=32
    local.get 5
    local.get 3
    i32.store offset=28
    local.get 5
    local.get 4
    i32.store offset=24
    local.get 5
    local.get 5
    i32.load offset=24
    local.get 5
    i32.load offset=40
    local.tee 0
    i32.load offset=12
    i32.shl
    i32.store offset=20
    local.get 5
    local.get 5
    i32.load offset=24
    local.get 0
    i32.load offset=8
    i32.const 1
    i32.sub
    i32.rem_u
    i32.const 1
    i32.add
    i32.store offset=16
    local.get 5
    local.get 5
    i32.load offset=16
    i32.store offset=12
    block  ;; label = @1
      loop  ;; label = @2
        local.get 5
        local.get 0
        i32.load
        local.get 5
        i32.load offset=12
        i32.const 2
        i32.shl
        i32.add
        i32.load
        i32.store offset=8
        block  ;; label = @3
          local.get 5
          i32.load offset=8
          br_if 0 (;@3;)
          local.get 5
          local.get 5
          i32.load offset=12
          i32.const -1
          i32.xor
          i32.store offset=44
          br 2 (;@1;)
        end
        block  ;; label = @3
          local.get 5
          i32.load offset=8
          local.get 0
          i32.load offset=16
          i32.const -1
          i32.xor
          i32.and
          local.get 5
          i32.load offset=20
          i32.eq
          i32.const 1
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 5
          local.get 5
          i32.load offset=8
          local.get 0
          i32.load offset=16
          i32.and
          i32.const 1
          i32.sub
          i32.store offset=4
          block  ;; label = @4
            local.get 5
            i32.load offset=36
            local.get 5
            i32.load offset=4
            i32.const 1
            i32.shl
            i32.add
            local.get 5
            i32.load offset=32
            local.get 5
            i32.load offset=28
            i32.const 1
            i32.shl
            i32.add
            local.get 0
            i32.load offset=20
            call 70
            i32.const 1
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 5
            local.get 5
            i32.load offset=12
            i32.store offset=44
            br 3 (;@1;)
          end
        end
        local.get 5
        local.get 0
        local.get 5
        i32.load offset=16
        local.get 5
        i32.load offset=12
        call 71
        i32.store offset=12
        br 0 (;@2;)
      end
    end
    local.get 5
    i32.load offset=44
    local.set 0
    local.get 5
    i32.const 48
    i32.add
    global.set 0
    local.get 0)
  (func (;70;) (type 4) (param i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    local.get 0
    i32.store offset=12
    local.get 3
    local.get 1
    i32.store offset=8
    local.get 3
    local.get 2
    i32.store offset=4
    loop  ;; label = @1
      i32.const 0
      local.set 0
      block  ;; label = @2
        local.get 3
        i32.load offset=4
        i32.const 0
        i32.gt_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.load offset=12
        i32.load16_u
        i32.const 65535
        i32.and
        local.get 3
        i32.load offset=8
        i32.load16_u
        i32.const 65535
        i32.and
        i32.eq
        local.set 0
      end
      block  ;; label = @2
        local.get 0
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        local.get 3
        i32.load offset=12
        i32.const 2
        i32.add
        i32.store offset=12
        local.get 3
        local.get 3
        i32.load offset=8
        i32.const 2
        i32.add
        i32.store offset=8
        local.get 3
        local.get 3
        i32.load offset=4
        i32.const -1
        i32.add
        i32.store offset=4
        br 1 (;@1;)
      end
    end
    local.get 3
    i32.load offset=4
    i32.const 0
    i32.eq
    i32.const 1
    i32.and)
  (func (;71;) (type 4) (param i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    local.get 0
    i32.store offset=12
    local.get 3
    local.get 1
    i32.store offset=8
    local.get 3
    local.get 2
    i32.store offset=4
    local.get 3
    i32.load offset=4
    local.get 3
    i32.load offset=8
    i32.add
    local.get 3
    i32.load offset=12
    i32.load offset=8
    i32.rem_s)
  (func (;72;) (type 11) (param i32 i32 i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 5
    global.set 0
    local.get 5
    local.get 0
    i32.store offset=28
    local.get 5
    local.get 1
    i32.store offset=24
    local.get 5
    local.get 2
    i32.store offset=20
    local.get 5
    local.get 3
    i32.store offset=16
    local.get 5
    local.get 4
    i32.store offset=12
    local.get 5
    local.get 5
    i32.load offset=28
    local.tee 0
    local.get 5
    i32.load offset=24
    local.get 5
    i32.load offset=24
    local.get 5
    i32.load offset=20
    local.get 5
    i32.load offset=16
    call 75
    i32.store offset=8
    block  ;; label = @1
      local.get 5
      i32.load offset=8
      i32.const 0
      i32.lt_s
      i32.const 1
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load
      local.get 5
      i32.load offset=8
      i32.const -1
      i32.xor
      i32.const 2
      i32.shl
      i32.add
      local.get 5
      i32.load offset=16
      local.get 0
      i32.load offset=12
      i32.shl
      local.get 5
      i32.load offset=12
      i32.const 1
      i32.add
      i32.or
      i32.store
    end
    local.get 5
    i32.const 32
    i32.add
    global.set 0)
  (func (;73;) (type 3) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    local.get 0
    i32.store offset=12
    local.get 2
    local.get 1
    i32.store offset=8
    local.get 2
    i32.load offset=12
    local.set 0
    local.get 2
    local.get 2
    i32.load offset=8
    i32.store offset=4
    local.get 2
    i32.const 1
    i32.store
    block  ;; label = @1
      loop  ;; label = @2
        local.get 2
        i32.load
        local.get 0
        i32.load offset=20
        i32.lt_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 2
        local.get 2
        i32.load offset=4
        i32.const 37
        i32.mul
        local.get 2
        i32.load offset=8
        i32.add
        i32.store offset=4
        local.get 2
        local.get 2
        i32.load
        i32.const 1
        i32.add
        i32.store
        br 0 (;@2;)
      end
    end
    local.get 2
    i32.load offset=4)
  (func (;74;) (type 6) (param i32 i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 4
    global.set 0
    local.get 4
    local.get 0
    i32.store offset=40
    local.get 4
    local.get 1
    i32.store offset=36
    local.get 4
    local.get 2
    i32.store offset=32
    local.get 4
    local.get 3
    i32.store offset=28
    local.get 4
    local.get 4
    i32.load offset=28
    local.get 4
    i32.load offset=40
    local.tee 0
    i32.load offset=12
    i32.shl
    i32.store offset=24
    local.get 4
    local.get 4
    i32.load offset=28
    local.get 0
    i32.load offset=8
    i32.const 1
    i32.sub
    i32.rem_u
    i32.const 1
    i32.add
    i32.store offset=20
    local.get 4
    local.get 4
    i32.load offset=20
    i32.store offset=16
    block  ;; label = @1
      loop  ;; label = @2
        local.get 4
        local.get 0
        i32.load
        local.get 4
        i32.load offset=16
        i32.const 2
        i32.shl
        i32.add
        i32.load
        i32.store offset=12
        block  ;; label = @3
          local.get 4
          i32.load offset=12
          br_if 0 (;@3;)
          local.get 4
          local.get 4
          i32.load offset=16
          i32.const -1
          i32.xor
          i32.store offset=44
          br 2 (;@1;)
        end
        block  ;; label = @3
          local.get 4
          i32.load offset=12
          local.get 0
          i32.load offset=16
          i32.const -1
          i32.xor
          i32.and
          local.get 4
          i32.load offset=24
          i32.eq
          i32.const 1
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          local.get 4
          i32.load offset=12
          local.get 0
          i32.load offset=16
          i32.and
          i32.const 1
          i32.sub
          i32.store offset=8
          block  ;; label = @4
            local.get 4
            i32.load offset=36
            local.get 4
            i32.load offset=8
            i32.const 2
            i32.shl
            i32.add
            local.get 0
            i32.load offset=20
            local.get 4
            i32.load offset=32
            call 47
            i32.const 1
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 4
            local.get 4
            i32.load offset=16
            i32.store offset=44
            br 3 (;@1;)
          end
        end
        local.get 4
        local.get 0
        local.get 4
        i32.load offset=20
        local.get 4
        i32.load offset=16
        call 71
        i32.store offset=16
        br 0 (;@2;)
      end
    end
    local.get 4
    i32.load offset=44
    local.set 0
    local.get 4
    i32.const 48
    i32.add
    global.set 0
    local.get 0)
  (func (;75;) (type 10) (param i32 i32 i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 5
    global.set 0
    local.get 5
    local.get 0
    i32.store offset=40
    local.get 5
    local.get 1
    i32.store offset=36
    local.get 5
    local.get 2
    i32.store offset=32
    local.get 5
    local.get 3
    i32.store offset=28
    local.get 5
    local.get 4
    i32.store offset=24
    local.get 5
    local.get 5
    i32.load offset=24
    local.get 5
    i32.load offset=40
    local.tee 0
    i32.load offset=12
    i32.shl
    i32.store offset=20
    local.get 5
    local.get 5
    i32.load offset=24
    local.get 0
    i32.load offset=8
    i32.const 1
    i32.sub
    i32.rem_u
    i32.const 1
    i32.add
    i32.store offset=16
    local.get 5
    local.get 5
    i32.load offset=16
    i32.store offset=12
    block  ;; label = @1
      loop  ;; label = @2
        local.get 5
        local.get 0
        i32.load
        local.get 5
        i32.load offset=12
        i32.const 2
        i32.shl
        i32.add
        i32.load
        i32.store offset=8
        block  ;; label = @3
          local.get 5
          i32.load offset=8
          br_if 0 (;@3;)
          local.get 5
          local.get 5
          i32.load offset=12
          i32.const -1
          i32.xor
          i32.store offset=44
          br 2 (;@1;)
        end
        block  ;; label = @3
          local.get 5
          i32.load offset=8
          local.get 0
          i32.load offset=16
          i32.const -1
          i32.xor
          i32.and
          local.get 5
          i32.load offset=20
          i32.eq
          i32.const 1
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 5
          local.get 5
          i32.load offset=8
          local.get 0
          i32.load offset=16
          i32.and
          i32.const 1
          i32.sub
          i32.store offset=4
          block  ;; label = @4
            local.get 5
            i32.load offset=36
            local.get 5
            i32.load offset=4
            i32.const 2
            i32.shl
            i32.add
            local.get 5
            i32.load offset=32
            local.get 5
            i32.load offset=28
            i32.const 2
            i32.shl
            i32.add
            local.get 0
            i32.load offset=20
            call 76
            i32.const 1
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 5
            local.get 5
            i32.load offset=12
            i32.store offset=44
            br 3 (;@1;)
          end
        end
        local.get 5
        local.get 0
        local.get 5
        i32.load offset=16
        local.get 5
        i32.load offset=12
        call 71
        i32.store offset=12
        br 0 (;@2;)
      end
    end
    local.get 5
    i32.load offset=44
    local.set 0
    local.get 5
    i32.const 48
    i32.add
    global.set 0
    local.get 0)
  (func (;76;) (type 4) (param i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    local.get 0
    i32.store offset=12
    local.get 3
    local.get 1
    i32.store offset=8
    local.get 3
    local.get 2
    i32.store offset=4
    loop  ;; label = @1
      i32.const 0
      local.set 0
      block  ;; label = @2
        local.get 3
        i32.load offset=4
        i32.const 0
        i32.gt_s
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.load offset=12
        i32.load
        local.get 3
        i32.load offset=8
        i32.load
        i32.eq
        local.set 0
      end
      block  ;; label = @2
        local.get 0
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        local.get 3
        i32.load offset=12
        i32.const 4
        i32.add
        i32.store offset=12
        local.get 3
        local.get 3
        i32.load offset=8
        i32.const 4
        i32.add
        i32.store offset=8
        local.get 3
        local.get 3
        i32.load offset=4
        i32.const -1
        i32.add
        i32.store offset=4
        br 1 (;@1;)
      end
    end
    local.get 3
    i32.load offset=4
    i32.const 0
    i32.eq
    i32.const 1
    i32.and)
  (func (;77;) (type 5) (result i32)
    i32.const 1
    i32.const 4
    call 6)
  (func (;78;) (type 1) (param i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    local.get 0
    i32.store offset=12
    local.get 1
    i32.load offset=12
    i32.load)
  (func (;79;) (type 1) (param i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    local.get 0
    i32.store offset=12
    local.get 1
    i32.load offset=12
    i32.load offset=16)
  (func (;80;) (type 1) (param i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    local.get 0
    i32.store offset=12
    local.get 1
    i32.load offset=12
    i32.load16_u offset=20
    i32.const 65535
    i32.and)
  (func (;81;) (type 1) (param i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    local.get 0
    i32.store offset=12
    local.get 1
    i32.load offset=12
    i32.load16_u offset=30
    i32.const 65535
    i32.and)
  (func (;82;) (type 1) (param i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    local.get 0
    i32.store offset=12
    local.get 1
    i32.load offset=12
    i32.load offset=32)
  (func (;83;) (type 1) (param i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    local.get 0
    i32.store offset=12
    local.get 1
    i32.load offset=12
    i32.load offset=36)
  (func (;84;) (type 1) (param i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    local.get 0
    i32.store offset=12
    local.get 1
    i32.load offset=12
    i32.load)
  (func (;85;) (type 1) (param i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    local.get 0
    i32.store offset=12
    local.get 1
    i32.load offset=12
    i32.load offset=8)
  (func (;86;) (type 1) (param i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    local.get 0
    i32.store offset=12
    local.get 1
    i32.load offset=12
    i32.load offset=4)
  (func (;87;) (type 1) (param i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    local.get 0
    i32.store offset=12
    local.get 1
    i32.load offset=12
    i32.load offset=12)
  (table (;0;) 1 1 funcref)
  (memory (;0;) 2)
  (global (;0;) (mut i32) (i32.const 67088))
  (export "memory" (memory 0))
  (export "_initialize" (func 1))
  (export "ucptrie_close" (func 13))
  (export "umutablecptrie_open" (func 14))
  (export "umutablecptrie_close" (func 26))
  (export "umutablecptrie_set" (func 27))
  (export "umutablecptrie_buildImmutable" (func 33))
  (export "create_uerrorcode" (func 77))
  (export "read_uerrorcode" (func 78))
  (export "read_ucptrie_highStart" (func 79))
  (export "read_ucptrie_shifted12HighStart" (func 80))
  (export "read_ucptrie_index3NullOffset" (func 81))
  (export "read_ucptrie_dataNullOffset" (func 82))
  (export "read_ucptrie_nullValue" (func 83))
  (export "get_index_ptr" (func 84))
  (export "get_index_length" (func 85))
  (export "get_data_ptr" (func 86))
  (export "get_data_length" (func 87))
  (data (;0;) (i32.const 1024) "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"))
