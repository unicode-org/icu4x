(module
  (type (;0;) (func (param i32 i32 i32) (result i32)))
  (type (;1;) (func (param i32 i32) (result i32)))
  (type (;2;) (func (param i32 i32)))
  (type (;3;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;4;) (func (param i32) (result i32)))
  (type (;5;) (func (param i32 i32 i32 i32)))
  (type (;6;) (func))
  (type (;7;) (func (param i32 i32 i32 i32 i32)))
  (type (;8;) (func (param i32 i32 i32)))
  (type (;9;) (func (param i32)))
  (type (;10;) (func (param i32 i32 i32 i32 i32 i32) (result i32)))
  (type (;11;) (func (param i64 i32 i32) (result i32)))
  (type (;12;) (func (param i32) (result i64)))
  (type (;13;) (func (param i32 i32 i32 i32 i32 i32 i32) (result i32)))
  (func $__rg_alloc (type 1) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17h23ff858d8c82b51eE)
  (func $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17h23ff858d8c82b51eE (type 1) (param i32 i32) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 0
    i32.load offset=15304
    i32.store offset=12
    block  ;; label = @1
      local.get 0
      i32.const 3
      i32.add
      i32.const 2
      i32.shr_u
      local.tee 3
      local.get 1
      local.get 2
      i32.const 12
      i32.add
      call $_ZN9wee_alloc15alloc_first_fit17h9bc4bd4c3d54ee85E
      local.tee 0
      br_if 0 (;@1;)
      local.get 2
      local.get 3
      local.get 1
      call $_ZN70_$LT$wee_alloc..LargeAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$22new_cell_for_free_list17hfd780db55d8403d7E
      i32.const 0
      local.set 0
      local.get 2
      i32.load
      br_if 0 (;@1;)
      local.get 2
      i32.load offset=4
      local.tee 0
      local.get 2
      i32.load offset=12
      i32.store offset=8
      local.get 2
      local.get 0
      i32.store offset=12
      local.get 3
      local.get 1
      local.get 2
      i32.const 12
      i32.add
      call $_ZN9wee_alloc15alloc_first_fit17h9bc4bd4c3d54ee85E
      local.set 0
    end
    i32.const 0
    local.get 2
    i32.load offset=12
    i32.store offset=15304
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 0)
  (func $__rg_dealloc (type 2) (param i32 i32)
    local.get 0
    local.get 1
    call $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17hb16969ed6285cab6E)
  (func $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17hb16969ed6285cab6E (type 2) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      i32.const 0
      i32.load offset=15304
      local.set 2
      local.get 0
      i32.const 0
      i32.store
      local.get 0
      i32.const -8
      i32.add
      local.tee 3
      local.get 3
      i32.load
      local.tee 4
      i32.const -2
      i32.and
      i32.store
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.const -4
                i32.add
                local.tee 5
                i32.load
                i32.const -4
                i32.and
                local.tee 6
                i32.eqz
                br_if 0 (;@6;)
                local.get 6
                i32.load
                local.tee 7
                i32.const 1
                i32.and
                i32.eqz
                br_if 1 (;@5;)
              end
              local.get 4
              i32.const -4
              i32.and
              local.tee 6
              i32.eqz
              br_if 1 (;@4;)
              i32.const 0
              local.get 6
              local.get 4
              i32.const 2
              i32.and
              select
              local.tee 4
              i32.eqz
              br_if 1 (;@4;)
              local.get 4
              i32.load8_u
              i32.const 1
              i32.and
              br_if 1 (;@4;)
              local.get 0
              local.get 4
              i32.load offset=8
              i32.const -4
              i32.and
              i32.store
              local.get 4
              local.get 3
              i32.const 1
              i32.or
              i32.store offset=8
              br 2 (;@3;)
            end
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 4
                  i32.const -4
                  i32.and
                  local.tee 0
                  br_if 0 (;@7;)
                  local.get 6
                  local.set 8
                  br 1 (;@6;)
                end
                local.get 6
                local.set 8
                i32.const 0
                local.get 0
                local.get 4
                i32.const 2
                i32.and
                select
                local.tee 4
                i32.eqz
                br_if 0 (;@6;)
                local.get 4
                local.get 4
                i32.load offset=4
                i32.const 3
                i32.and
                local.get 6
                i32.or
                i32.store offset=4
                local.get 5
                i32.load
                local.tee 0
                i32.const -4
                i32.and
                local.tee 8
                i32.eqz
                br_if 1 (;@5;)
                local.get 3
                i32.load
                i32.const -4
                i32.and
                local.set 0
                local.get 8
                i32.load
                local.set 7
              end
              local.get 8
              local.get 7
              i32.const 3
              i32.and
              local.get 0
              i32.or
              i32.store
              local.get 5
              i32.load
              local.set 0
            end
            local.get 5
            local.get 0
            i32.const 3
            i32.and
            i32.store
            local.get 3
            local.get 3
            i32.load
            local.tee 0
            i32.const 3
            i32.and
            i32.store
            local.get 0
            i32.const 2
            i32.and
            i32.eqz
            br_if 1 (;@3;)
            local.get 6
            local.get 6
            i32.load
            i32.const 2
            i32.or
            i32.store
            br 1 (;@3;)
          end
          local.get 0
          local.get 2
          i32.store
          br 1 (;@2;)
        end
        local.get 2
        local.set 3
      end
      i32.const 0
      local.get 3
      i32.store offset=15304
    end)
  (func $__rg_realloc (type 3) (param i32 i32 i32 i32) (result i32)
    block  ;; label = @1
      local.get 3
      local.get 2
      call $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17h23ff858d8c82b51eE
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      local.get 0
      local.get 3
      local.get 1
      local.get 1
      local.get 3
      i32.gt_u
      select
      call $memcpy
      drop
      local.get 0
      local.get 1
      call $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17hb16969ed6285cab6E
    end
    local.get 2)
  (func $_ZN74_$LT$unicode_bmp_blocks_selector..BMPBlock$u20$as$u20$core..fmt..Debug$GT$3fmt17h5f69e6c0e6b743a2E (type 1) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            i32.load8_u
            br_table 1 (;@3;) 2 (;@2;) 0 (;@4;) 1 (;@3;)
          end
          local.get 2
          local.get 1
          i32.load offset=24
          i32.const 10368
          i32.const 7
          local.get 1
          i32.const 28
          i32.add
          i32.load
          i32.load offset=12
          call_indirect (type 0)
          i32.store8 offset=8
          local.get 2
          local.get 1
          i32.store
          local.get 2
          i32.const 0
          i32.store8 offset=9
          local.get 2
          i32.const 0
          i32.store offset=4
          br 2 (;@1;)
        end
        local.get 2
        local.get 1
        i32.load offset=24
        i32.const 10391
        i32.const 5
        local.get 1
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 0)
        i32.store8 offset=8
        local.get 2
        local.get 1
        i32.store
        local.get 2
        i32.const 0
        i32.store8 offset=9
        local.get 2
        i32.const 0
        i32.store offset=4
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.load offset=24
      i32.const 10375
      i32.const 16
      local.get 1
      i32.const 28
      i32.add
      i32.load
      i32.load offset=12
      call_indirect (type 0)
      i32.store8 offset=8
      local.get 2
      local.get 1
      i32.store
      local.get 2
      i32.const 0
      i32.store8 offset=9
      local.get 2
      i32.const 0
      i32.store offset=4
    end
    local.get 2
    call $_ZN4core3fmt8builders10DebugTuple6finish17h45fab94fff212740E
    local.set 1
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 1)
  (func $_ZN4core3fmt8builders10DebugTuple6finish17h45fab94fff212740E (type 4) (param i32) (result i32)
    (local i32 i32 i32)
    local.get 0
    i32.load8_u offset=8
    local.set 1
    block  ;; label = @1
      local.get 0
      i32.load offset=4
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const 255
      i32.and
      local.set 3
      i32.const 1
      local.set 1
      block  ;; label = @2
        local.get 3
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 2
          i32.const 1
          i32.ne
          br_if 0 (;@3;)
          local.get 0
          i32.load8_u offset=9
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          i32.load
          local.tee 3
          i32.load8_u
          i32.const 4
          i32.and
          br_if 0 (;@3;)
          i32.const 1
          local.set 1
          local.get 3
          i32.load offset=24
          i32.const 14452
          i32.const 1
          local.get 3
          i32.const 28
          i32.add
          i32.load
          i32.load offset=12
          call_indirect (type 0)
          br_if 1 (;@2;)
        end
        local.get 0
        i32.load
        local.tee 1
        i32.load offset=24
        i32.const 15196
        i32.const 1
        local.get 1
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 0)
        local.set 1
      end
      local.get 0
      local.get 1
      i32.store8 offset=8
    end
    local.get 1
    i32.const 255
    i32.and
    i32.const 0
    i32.ne)
  (func $main (type 1) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64)
    global.get 0
    i32.const 128
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          i32.const 40
          i32.const 4
          call $__rust_alloc
          local.tee 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          i32.const 0
          i32.store offset=96
          local.get 2
          i64.const 4
          i64.store offset=88
          local.get 2
          i32.const 88
          i32.add
          i32.const 10240
          call $_ZN10icu_uniset7builder17UnicodeSetBuilder9add_range17hdb84232bbfb68c6cE
          local.get 2
          i32.const 56
          i32.add
          i32.const 8
          i32.add
          local.tee 4
          local.get 2
          i32.load offset=96
          i32.store
          local.get 2
          local.get 2
          i64.load offset=88
          i64.store offset=56
          local.get 2
          i32.const 104
          i32.add
          local.get 2
          i32.const 56
          i32.add
          call $_ZN10icu_uniset7builder17UnicodeSetBuilder5build17he37188db2fe05d25E
          local.get 2
          i32.const 32
          i32.add
          i32.const 11
          i32.add
          local.get 2
          i32.const 104
          i32.add
          i32.const 8
          i32.add
          local.tee 5
          i64.load
          i64.store align=1
          local.get 2
          local.get 2
          i64.load offset=104
          i64.store offset=35 align=1
          local.get 2
          i32.const 0
          i32.store offset=80
          local.get 2
          i64.const 4
          i64.store offset=72
          local.get 2
          i32.const 72
          i32.add
          i32.const 10252
          call $_ZN10icu_uniset7builder17UnicodeSetBuilder9add_range17hdb84232bbfb68c6cE
          local.get 2
          i32.const 88
          i32.add
          i32.const 8
          i32.add
          local.get 2
          i32.load offset=80
          i32.store
          local.get 2
          local.get 2
          i64.load offset=72
          i64.store offset=88
          local.get 2
          i32.const 56
          i32.add
          local.get 2
          i32.const 88
          i32.add
          call $_ZN10icu_uniset7builder17UnicodeSetBuilder5build17he37188db2fe05d25E
          local.get 2
          i32.const 104
          i32.add
          i32.const 11
          i32.add
          local.get 4
          i64.load
          i64.store align=1
          local.get 2
          local.get 2
          i64.load offset=56
          i64.store offset=107 align=1
          local.get 3
          i32.const 0
          i32.store8
          local.get 3
          local.get 2
          i64.load offset=32 align=1
          i64.store offset=1 align=1
          local.get 3
          i32.const 9
          i32.add
          local.get 2
          i32.const 32
          i32.add
          i32.const 8
          i32.add
          i64.load align=1
          i64.store align=1
          local.get 3
          i32.const 16
          i32.add
          local.get 2
          i32.const 32
          i32.add
          i32.const 15
          i32.add
          i32.load align=1
          i32.store align=1
          local.get 3
          i32.const 1
          i32.store8 offset=20
          local.get 3
          local.get 2
          i64.load offset=104 align=1
          i64.store offset=21 align=1
          local.get 3
          i32.const 29
          i32.add
          local.get 5
          i64.load align=1
          i64.store align=1
          local.get 3
          i32.const 36
          i32.add
          local.get 2
          i32.const 104
          i32.add
          i32.const 15
          i32.add
          i32.load align=1
          i32.store align=1
          i32.const 10264
          local.set 6
          i32.const 4
          local.set 7
          i32.const 0
          local.set 8
          i32.const 0
          local.set 9
          block  ;; label = @4
            loop  ;; label = @5
              local.get 6
              i32.const 1
              i32.add
              local.set 5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 6
                  i32.load8_s
                  local.tee 4
                  i32.const -1
                  i32.le_s
                  br_if 0 (;@7;)
                  local.get 4
                  i32.const 255
                  i32.and
                  local.set 10
                  local.get 5
                  local.set 6
                  br 1 (;@6;)
                end
                local.get 4
                i32.const 31
                i32.and
                local.set 11
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 5
                            i32.const 10302
                            i32.eq
                            br_if 0 (;@12;)
                            local.get 6
                            i32.load8_u offset=1
                            i32.const 63
                            i32.and
                            local.set 5
                            local.get 6
                            i32.const 2
                            i32.add
                            local.set 12
                            local.get 4
                            i32.const 255
                            i32.and
                            i32.const 223
                            i32.gt_u
                            br_if 1 (;@11;)
                            local.get 11
                            i32.const 6
                            i32.shl
                            local.get 5
                            i32.or
                            local.set 10
                            local.get 12
                            local.set 6
                            br 6 (;@6;)
                          end
                          i32.const 10302
                          local.set 6
                          i32.const 0
                          local.set 5
                          local.get 4
                          i32.const 255
                          i32.and
                          i32.const 223
                          i32.le_u
                          br_if 4 (;@7;)
                          i32.const 10302
                          local.set 13
                          br 1 (;@10;)
                        end
                        i32.const 10302
                        local.set 13
                        local.get 12
                        i32.const 10302
                        i32.ne
                        br_if 1 (;@9;)
                      end
                      i32.const 0
                      local.set 12
                      br 1 (;@8;)
                    end
                    local.get 6
                    i32.const 3
                    i32.add
                    local.set 13
                    local.get 6
                    i32.load8_u offset=2
                    i32.const 63
                    i32.and
                    local.set 12
                  end
                  local.get 12
                  local.get 5
                  i32.const 6
                  i32.shl
                  i32.or
                  local.set 5
                  block  ;; label = @8
                    local.get 4
                    i32.const 255
                    i32.and
                    i32.const 240
                    i32.ge_u
                    br_if 0 (;@8;)
                    local.get 5
                    local.get 11
                    i32.const 12
                    i32.shl
                    i32.or
                    local.set 10
                    local.get 13
                    local.set 6
                    br 2 (;@6;)
                  end
                  i32.const 10302
                  local.set 6
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 13
                      i32.const 10302
                      i32.ne
                      br_if 0 (;@9;)
                      i32.const 0
                      local.set 4
                      br 1 (;@8;)
                    end
                    local.get 13
                    i32.const 1
                    i32.add
                    local.set 6
                    local.get 13
                    i32.load8_u
                    i32.const 63
                    i32.and
                    local.set 4
                  end
                  local.get 5
                  i32.const 6
                  i32.shl
                  local.get 11
                  i32.const 18
                  i32.shl
                  i32.const 1835008
                  i32.and
                  i32.or
                  local.get 4
                  i32.or
                  local.tee 10
                  i32.const 1114112
                  i32.ne
                  br_if 1 (;@6;)
                  br 6 (;@1;)
                end
                local.get 11
                i32.const 6
                i32.shl
                local.set 10
              end
              i32.const -20
              local.set 5
              local.get 3
              local.set 4
              block  ;; label = @6
                loop  ;; label = @7
                  block  ;; label = @8
                    local.get 5
                    i32.const 20
                    i32.ne
                    br_if 0 (;@8;)
                    i32.const 2
                    local.set 4
                    br 2 (;@6;)
                  end
                  local.get 2
                  i32.const 16
                  i32.add
                  local.get 4
                  i32.load offset=4
                  local.get 4
                  i32.load offset=12
                  local.tee 12
                  local.get 10
                  call $_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13binary_search17h5d3d7656eba25190E
                  local.get 2
                  i32.load offset=20
                  local.tee 14
                  i32.const 1
                  i32.and
                  local.set 13
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 2
                      i32.load offset=16
                      i32.eqz
                      br_if 0 (;@9;)
                      i32.const 0
                      local.set 11
                      local.get 13
                      i32.eqz
                      br_if 1 (;@8;)
                      local.get 14
                      local.get 12
                      i32.lt_u
                      local.set 11
                      br 1 (;@8;)
                    end
                    local.get 13
                    i32.const 1
                    i32.xor
                    local.set 11
                  end
                  local.get 4
                  i32.const 20
                  i32.add
                  local.set 4
                  local.get 5
                  i32.const 20
                  i32.add
                  local.set 5
                  local.get 11
                  i32.eqz
                  br_if 0 (;@7;)
                end
                local.get 3
                local.get 5
                i32.add
                i32.load8_u
                local.set 4
              end
              block  ;; label = @6
                block  ;; label = @7
                  local.get 9
                  local.get 8
                  i32.ne
                  br_if 0 (;@7;)
                  local.get 8
                  i32.const 1
                  i32.add
                  local.tee 5
                  local.get 8
                  i32.lt_u
                  br_if 3 (;@4;)
                  local.get 8
                  i32.const 1
                  i32.shl
                  local.tee 11
                  local.get 5
                  local.get 11
                  local.get 5
                  i32.gt_u
                  select
                  local.tee 5
                  i32.const 4
                  local.get 5
                  i32.const 4
                  i32.gt_u
                  select
                  local.tee 5
                  i32.const 536870911
                  i32.and
                  local.get 5
                  i32.eq
                  i32.const 2
                  i32.shl
                  local.set 11
                  local.get 5
                  i32.const 3
                  i32.shl
                  local.set 5
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 8
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 2
                      i32.const 4
                      i32.store offset=112
                      local.get 2
                      local.get 7
                      i32.store offset=104
                      local.get 2
                      local.get 8
                      i32.const 3
                      i32.shl
                      i32.store offset=108
                      br 1 (;@8;)
                    end
                    local.get 2
                    i32.const 0
                    i32.store offset=104
                  end
                  local.get 2
                  i32.const 32
                  i32.add
                  local.get 5
                  local.get 11
                  local.get 2
                  i32.const 104
                  i32.add
                  call $_ZN5alloc7raw_vec11finish_grow17h3e91ad6198eee436E
                  local.get 2
                  i32.load offset=36
                  local.set 7
                  local.get 2
                  i32.load offset=40
                  local.set 11
                  local.get 2
                  i32.load offset=32
                  i32.const 1
                  i32.eq
                  br_if 1 (;@6;)
                  local.get 11
                  i32.const 3
                  i32.shr_u
                  local.set 8
                end
                local.get 7
                local.get 9
                i32.const 3
                i32.shl
                i32.add
                local.tee 5
                local.get 4
                i32.store8 offset=4
                local.get 5
                local.get 10
                i32.store
                local.get 9
                i32.const 1
                i32.add
                local.set 9
                local.get 6
                i32.const 10302
                i32.ne
                br_if 1 (;@5;)
                br 5 (;@1;)
              end
            end
            local.get 7
            local.set 5
            br 2 (;@2;)
          end
          i32.const 0
          local.set 11
          br 1 (;@2;)
        end
        i32.const 40
        i32.const 4
        call $_ZN5alloc5alloc18handle_alloc_error17h8e32739f1fa5c58cE
        unreachable
      end
      block  ;; label = @2
        local.get 11
        i32.eqz
        br_if 0 (;@2;)
        local.get 5
        local.get 11
        call $_ZN5alloc5alloc18handle_alloc_error17h8e32739f1fa5c58cE
        unreachable
      end
      call $_ZN5alloc7raw_vec17capacity_overflow17hb5aace515ee70317E
      unreachable
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 9
        i32.eqz
        br_if 0 (;@2;)
        local.get 9
        i32.const 3
        i32.shl
        local.set 5
        local.get 7
        local.set 4
        loop  ;; label = @3
          local.get 4
          i64.load align=4
          local.tee 15
          i32.wrap_i64
          local.tee 11
          i32.const 1114112
          i32.eq
          br_if 1 (;@2;)
          local.get 2
          local.get 11
          i32.store offset=72
          local.get 2
          local.get 15
          i64.const 32
          i64.shr_u
          i64.store8 offset=31
          local.get 2
          i32.const 1
          i32.store offset=44
          local.get 2
          i32.const 2
          i32.store offset=36
          local.get 2
          local.get 2
          i32.const 31
          i32.add
          i32.store offset=40
          local.get 2
          local.get 2
          i32.const 72
          i32.add
          i32.store offset=32
          local.get 2
          i32.const 8
          i32.add
          i32.const 0
          call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in17h31198357b084008bE
          local.get 2
          i32.const 0
          i32.store offset=64
          local.get 2
          local.get 2
          i64.load offset=8
          i64.store offset=56
          local.get 2
          local.get 2
          i32.const 56
          i32.add
          i32.store offset=88
          local.get 2
          i32.const 2
          i32.store offset=124
          local.get 2
          i32.const 2
          i32.store offset=116
          local.get 2
          i32.const 10304
          i32.store offset=112
          local.get 2
          i32.const 2
          i32.store offset=108
          local.get 2
          i32.const 11256
          i32.store offset=104
          local.get 2
          local.get 2
          i32.const 32
          i32.add
          i32.store offset=120
          local.get 2
          i32.const 88
          i32.add
          i32.const 10736
          local.get 2
          i32.const 104
          i32.add
          call $_ZN4core3fmt5write17hbe0dd8e9c0d9aeb7E
          br_if 2 (;@1;)
          block  ;; label = @4
            local.get 2
            i32.load offset=60
            local.tee 11
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.load offset=56
            local.get 11
            call $__rust_dealloc
          end
          local.get 4
          i32.const 8
          i32.add
          local.set 4
          local.get 5
          i32.const -8
          i32.add
          local.tee 5
          br_if 0 (;@3;)
        end
      end
      block  ;; label = @2
        local.get 8
        i32.eqz
        br_if 0 (;@2;)
        local.get 8
        i32.const 3
        i32.shl
        local.tee 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 7
        local.get 4
        call $__rust_dealloc
      end
      i32.const 0
      local.set 4
      loop  ;; label = @2
        block  ;; label = @3
          local.get 3
          local.get 4
          i32.add
          local.tee 11
          i32.const 8
          i32.add
          i32.load
          local.tee 5
          i32.eqz
          br_if 0 (;@3;)
          local.get 5
          i32.const 2
          i32.shl
          local.tee 5
          i32.eqz
          br_if 0 (;@3;)
          local.get 11
          i32.const 4
          i32.add
          i32.load
          local.get 5
          call $__rust_dealloc
        end
        local.get 4
        i32.const 20
        i32.add
        local.tee 4
        i32.const 40
        i32.ne
        br_if 0 (;@2;)
      end
      local.get 3
      i32.const 40
      call $__rust_dealloc
      local.get 2
      i32.const 128
      i32.add
      global.set 0
      i32.const 0
      return
    end
    i32.const 10544
    i32.const 51
    local.get 2
    i32.const 104
    i32.add
    i32.const 10596
    i32.const 10612
    call $_ZN4core6result13unwrap_failed17hfa73799f0c5a0f1cE
    unreachable)
  (func $__rust_alloc (type 1) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $__rg_alloc)
  (func $_ZN10icu_uniset7builder17UnicodeSetBuilder9add_range17hdb84232bbfb68c6cE (type 2) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 80
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      local.get 1
      i32.load
      local.tee 3
      local.get 1
      i32.load offset=4
      local.tee 1
      i32.gt_u
      br_if 0 (;@1;)
      local.get 1
      i32.const 1
      i32.add
      local.set 4
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.load offset=8
                local.tee 5
                i32.eqz
                br_if 0 (;@6;)
                local.get 2
                i32.const 16
                i32.add
                local.get 0
                i32.load
                local.tee 6
                local.get 5
                local.get 3
                call $_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13binary_search17h5d3d7656eba25190E
                local.get 2
                i32.const 8
                i32.add
                local.get 6
                local.get 5
                local.get 4
                call $_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13binary_search17h5d3d7656eba25190E
                local.get 2
                i32.load offset=20
                local.tee 1
                i32.const 1
                i32.and
                local.set 7
                local.get 2
                i32.load offset=12
                local.set 8
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 2
                        i32.load offset=8
                        local.tee 9
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 7
                        br_if 0 (;@10;)
                        local.get 1
                        local.get 8
                        i32.eq
                        br_if 1 (;@9;)
                      end
                      block  ;; label = @10
                        local.get 7
                        br_if 0 (;@10;)
                        local.get 5
                        local.get 1
                        i32.le_u
                        br_if 6 (;@4;)
                        local.get 6
                        local.get 1
                        i32.const 2
                        i32.shl
                        i32.add
                        local.get 3
                        i32.store
                        local.get 1
                        i32.const 1
                        i32.add
                        local.set 1
                      end
                      local.get 8
                      i32.const 1
                      i32.and
                      br_if 2 (;@7;)
                      local.get 9
                      i32.eqz
                      br_if 1 (;@8;)
                      local.get 0
                      i32.load offset=8
                      local.tee 3
                      local.get 8
                      i32.const -1
                      i32.add
                      local.tee 8
                      i32.le_u
                      br_if 6 (;@3;)
                      local.get 0
                      i32.load
                      local.get 8
                      i32.const 2
                      i32.shl
                      i32.add
                      local.get 4
                      i32.store
                      br 2 (;@7;)
                    end
                    local.get 2
                    local.get 4
                    i32.store offset=28
                    local.get 2
                    local.get 3
                    i32.store offset=24
                    local.get 2
                    i32.const 32
                    i32.add
                    local.get 0
                    local.get 1
                    local.get 1
                    call $_ZN5alloc3vec12Vec$LT$T$GT$5drain17h8be220de23425f59E
                    local.get 2
                    i32.const 56
                    i32.add
                    local.get 2
                    i32.const 32
                    i32.add
                    i32.store
                    local.get 2
                    i32.const 44
                    i32.add
                    i32.load
                    local.set 0
                    local.get 2
                    i32.load offset=40
                    local.set 1
                    local.get 2
                    local.get 2
                    i32.const 24
                    i32.add
                    i32.store offset=52
                    block  ;; label = @9
                      local.get 1
                      local.get 0
                      i32.eq
                      br_if 0 (;@9;)
                      local.get 2
                      local.get 0
                      local.get 1
                      i32.sub
                      i32.const -4
                      i32.add
                      i32.const -4
                      i32.and
                      local.get 1
                      i32.add
                      i32.const 4
                      i32.add
                      i32.store offset=40
                    end
                    local.get 2
                    i32.load offset=36
                    i32.eqz
                    br_if 3 (;@5;)
                    local.get 2
                    i32.const 32
                    i32.add
                    local.get 2
                    i32.const 52
                    i32.add
                    local.tee 1
                    call $_ZN5alloc3vec14Drain$LT$T$GT$4fill17h8a131803b354a583E
                    i32.eqz
                    br_if 6 (;@2;)
                    block  ;; label = @9
                      local.get 2
                      i32.load offset=56
                      local.get 2
                      i32.load offset=52
                      i32.sub
                      local.tee 0
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 2
                      i32.load offset=48
                      local.tee 3
                      local.get 2
                      i32.load offset=36
                      local.get 2
                      i32.load offset=32
                      i32.add
                      local.get 0
                      i32.const 2
                      i32.shr_u
                      local.tee 0
                      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve17h05b20ec8d167cd79E
                      local.get 3
                      i32.load
                      local.tee 3
                      local.get 2
                      i32.load offset=32
                      local.tee 4
                      local.get 0
                      i32.add
                      local.tee 0
                      i32.const 2
                      i32.shl
                      i32.add
                      local.get 3
                      local.get 4
                      i32.const 2
                      i32.shl
                      i32.add
                      local.get 2
                      i32.load offset=36
                      i32.const 2
                      i32.shl
                      call $memmove
                      drop
                      local.get 2
                      local.get 0
                      i32.store offset=32
                      local.get 2
                      i32.const 32
                      i32.add
                      local.get 1
                      call $_ZN5alloc3vec14Drain$LT$T$GT$4fill17h8a131803b354a583E
                      i32.eqz
                      br_if 7 (;@2;)
                    end
                    local.get 2
                    i32.const 0
                    i32.store offset=72
                    local.get 2
                    i64.const 4
                    i64.store offset=64
                    local.get 2
                    i32.const 64
                    i32.add
                    local.get 1
                    call $_ZN80_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..SpecExtend$LT$T$C$I$GT$$GT$11spec_extend17hcbe54f7519367931E
                    local.get 2
                    i32.load offset=68
                    local.set 8
                    local.get 2
                    i32.load offset=64
                    local.set 6
                    block  ;; label = @9
                      local.get 2
                      i32.load offset=72
                      local.tee 1
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 2
                      i32.load offset=48
                      local.tee 0
                      local.get 2
                      i32.load offset=36
                      local.get 2
                      i32.load offset=32
                      i32.add
                      local.get 1
                      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve17h05b20ec8d167cd79E
                      local.get 0
                      i32.load
                      local.tee 3
                      local.get 2
                      i32.load offset=32
                      local.tee 4
                      local.get 1
                      i32.add
                      local.tee 0
                      i32.const 2
                      i32.shl
                      i32.add
                      local.get 3
                      local.get 4
                      i32.const 2
                      i32.shl
                      local.tee 5
                      i32.add
                      local.get 2
                      i32.load offset=36
                      i32.const 2
                      i32.shl
                      call $memmove
                      drop
                      local.get 2
                      local.get 0
                      i32.store offset=32
                      local.get 2
                      i32.load offset=48
                      local.tee 4
                      i32.load offset=8
                      local.tee 3
                      local.get 0
                      i32.eq
                      br_if 0 (;@9;)
                      local.get 3
                      i32.const 2
                      i32.shl
                      local.tee 0
                      local.get 5
                      i32.sub
                      local.set 5
                      local.get 1
                      i32.const 2
                      i32.shl
                      local.set 1
                      local.get 4
                      i32.load
                      local.get 0
                      i32.add
                      local.set 0
                      local.get 6
                      local.set 3
                      loop  ;; label = @10
                        local.get 1
                        i32.eqz
                        br_if 1 (;@9;)
                        local.get 0
                        local.get 3
                        i32.load
                        i32.store
                        local.get 4
                        local.get 4
                        i32.load offset=8
                        i32.const 1
                        i32.add
                        i32.store offset=8
                        local.get 3
                        i32.const 4
                        i32.add
                        local.set 3
                        local.get 0
                        i32.const 4
                        i32.add
                        local.set 0
                        local.get 5
                        local.get 1
                        i32.const -4
                        i32.add
                        local.tee 1
                        i32.ne
                        br_if 0 (;@10;)
                      end
                    end
                    local.get 8
                    i32.eqz
                    br_if 6 (;@2;)
                    local.get 8
                    i32.const 2
                    i32.shl
                    local.tee 1
                    i32.eqz
                    br_if 6 (;@2;)
                    local.get 6
                    local.get 1
                    call $__rust_dealloc
                    br 6 (;@2;)
                  end
                  local.get 8
                  i32.const 1
                  i32.add
                  local.set 8
                end
                local.get 1
                local.get 8
                i32.ge_u
                br_if 5 (;@1;)
                local.get 2
                i32.const 32
                i32.add
                local.get 0
                local.get 1
                local.get 8
                call $_ZN5alloc3vec12Vec$LT$T$GT$5drain17h8be220de23425f59E
                local.get 2
                i32.const 32
                i32.add
                call $_ZN68_$LT$alloc..vec..Drain$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hbb7c9400224cfddaE
                br 5 (;@1;)
              end
              local.get 2
              local.get 4
              i32.store offset=36
              local.get 2
              local.get 3
              i32.store offset=32
              local.get 0
              local.get 2
              i32.const 32
              i32.add
              i32.const 2
              call $_ZN5alloc3vec12Vec$LT$T$GT$17extend_from_slice17h032ebe9cd1c3b0dfE
              br 4 (;@1;)
            end
            local.get 2
            i32.load offset=48
            local.get 2
            i32.const 52
            i32.add
            call $_ZN80_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..SpecExtend$LT$T$C$I$GT$$GT$11spec_extend17hcbe54f7519367931E
            br 2 (;@2;)
          end
          local.get 1
          local.get 5
          i32.const 14892
          call $_ZN4core9panicking18panic_bounds_check17hc53ca9df0b8509fdE
          unreachable
        end
        local.get 8
        local.get 3
        i32.const 14908
        call $_ZN4core9panicking18panic_bounds_check17hc53ca9df0b8509fdE
        unreachable
      end
      local.get 2
      i32.const 32
      i32.add
      call $_ZN68_$LT$alloc..vec..Drain$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hbb7c9400224cfddaE
    end
    local.get 2
    i32.const 80
    i32.add
    global.set 0)
  (func $_ZN10icu_uniset7builder17UnicodeSetBuilder5build17he37188db2fe05d25E (type 2) (param i32 i32)
    (local i32 i32 i64 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 1
    i32.load
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i64.load offset=4 align=4
        local.tee 4
        i64.const 32
        i64.shr_u
        i32.wrap_i64
        local.tee 1
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 1
        i32.const 1
        i32.add
        local.set 5
        local.get 3
        local.set 6
        loop  ;; label = @3
          block  ;; label = @4
            local.get 5
            i32.const -1
            i32.add
            local.tee 5
            i32.const 2
            i32.ge_u
            br_if 0 (;@4;)
            local.get 1
            i32.eqz
            br_if 2 (;@2;)
            local.get 1
            i32.const 2
            i32.shl
            local.get 3
            i32.add
            i32.const -4
            i32.add
            i32.load
            i32.const 1114112
            i32.gt_u
            br_if 2 (;@2;)
            i32.const 0
            local.set 7
            local.get 3
            local.set 6
            loop  ;; label = @5
              local.get 1
              i32.const 2
              local.get 1
              i32.const 2
              i32.lt_u
              select
              local.tee 5
              i32.const 1
              i32.le_u
              br_if 4 (;@1;)
              local.get 6
              i32.load offset=4
              local.get 7
              i32.add
              local.get 6
              i32.load
              i32.sub
              local.set 7
              local.get 6
              local.get 5
              i32.const 2
              i32.shl
              i32.add
              local.set 6
              local.get 1
              local.get 5
              i32.sub
              local.tee 1
              br_if 0 (;@5;)
            end
            local.get 0
            local.get 7
            i32.store offset=12
            local.get 0
            local.get 4
            i64.store offset=4 align=4
            local.get 0
            local.get 3
            i32.store
            local.get 2
            i32.const 16
            i32.add
            global.set 0
            return
          end
          local.get 6
          i32.load
          local.set 7
          local.get 6
          i32.const 4
          i32.add
          local.tee 8
          local.set 6
          local.get 7
          local.get 8
          i32.load
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 2
      i32.const 12
      i32.add
      local.get 1
      i32.store
      local.get 2
      i32.const 0
      i32.store
      local.get 2
      local.get 4
      i64.const 32
      i64.shl
      local.get 3
      i64.extend_i32_u
      i64.or
      i64.store offset=4 align=4
      i32.const 15260
      i32.const 43
      local.get 2
      i32.const 14828
      i32.const 14844
      call $_ZN4core6result13unwrap_failed17hfa73799f0c5a0f1cE
      unreachable
    end
    i32.const 1
    local.get 5
    i32.const 14924
    call $_ZN4core9panicking18panic_bounds_check17hc53ca9df0b8509fdE
    unreachable)
  (func $_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13binary_search17h5d3d7656eba25190E (type 5) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32)
    i32.const 1
    local.set 4
    i32.const 0
    local.set 5
    i32.const 0
    local.set 6
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          br_table 2 (;@1;) 1 (;@2;) 0 (;@3;)
        end
        i32.const 0
        local.set 5
        loop  ;; label = @3
          local.get 5
          local.get 2
          i32.const 1
          i32.shr_u
          local.tee 6
          local.get 5
          i32.add
          local.tee 7
          local.get 1
          local.get 7
          i32.const 2
          i32.shl
          i32.add
          i32.load
          local.get 3
          i32.gt_u
          select
          local.set 5
          local.get 2
          local.get 6
          i32.sub
          local.tee 2
          i32.const 1
          i32.gt_u
          br_if 0 (;@3;)
        end
      end
      block  ;; label = @2
        local.get 1
        local.get 5
        i32.const 2
        i32.shl
        i32.add
        i32.load
        local.tee 2
        local.get 3
        i32.ne
        br_if 0 (;@2;)
        local.get 5
        local.set 6
        i32.const 0
        local.set 4
        br 1 (;@1;)
      end
      local.get 5
      local.get 2
      local.get 3
      i32.lt_u
      i32.add
      local.set 6
    end
    local.get 0
    local.get 6
    i32.store offset=4
    local.get 0
    local.get 4
    i32.store)
  (func $_ZN5alloc7raw_vec11finish_grow17h3e91ad6198eee436E (type 5) (param i32 i32 i32 i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 2
                i32.eqz
                br_if 0 (;@6;)
                local.get 1
                i32.const 0
                i32.lt_s
                br_if 1 (;@5;)
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 3
                      i32.load
                      local.tee 4
                      br_if 0 (;@9;)
                      local.get 1
                      i32.eqz
                      br_if 1 (;@8;)
                      br 5 (;@4;)
                    end
                    local.get 3
                    i32.load offset=4
                    local.tee 3
                    br_if 1 (;@7;)
                    local.get 1
                    br_if 4 (;@4;)
                  end
                  local.get 2
                  local.set 3
                  br 5 (;@2;)
                end
                local.get 4
                local.get 3
                local.get 2
                local.get 1
                call $__rust_realloc
                local.tee 3
                i32.eqz
                br_if 3 (;@3;)
                br 4 (;@2;)
              end
              local.get 0
              local.get 1
              i32.store offset=4
              local.get 0
              i32.const 1
              i32.store
              local.get 0
              i32.const 8
              i32.add
              i32.const 0
              i32.store
              return
            end
            local.get 0
            i32.const 1
            i32.store
            local.get 0
            i32.const 8
            i32.add
            i32.const 0
            i32.store
            return
          end
          local.get 1
          local.get 2
          call $__rust_alloc
          local.tee 3
          br_if 1 (;@2;)
        end
        local.get 0
        local.get 1
        i32.store offset=4
        i32.const 1
        local.set 3
        local.get 2
        local.set 1
        br 1 (;@1;)
      end
      local.get 0
      local.get 3
      i32.store offset=4
      i32.const 0
      local.set 3
    end
    local.get 0
    local.get 3
    i32.store
    local.get 0
    i32.const 8
    i32.add
    local.get 1
    i32.store)
  (func $_ZN5alloc5alloc18handle_alloc_error17h8e32739f1fa5c58cE (type 2) (param i32 i32)
    local.get 0
    local.get 1
    call $__rust_alloc_error_handler
    unreachable)
  (func $_ZN5alloc7raw_vec17capacity_overflow17hb5aace515ee70317E (type 6)
    i32.const 10396
    i32.const 17
    i32.const 10416
    call $_ZN4core9panicking5panic17h1f31dfdebac1729fE
    unreachable)
  (func $_ZN43_$LT$char$u20$as$u20$core..fmt..Display$GT$3fmt17h21bc6ab835592adfE (type 1) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.load offset=8
          i32.const 1
          i32.eq
          br_if 0 (;@3;)
          local.get 1
          i32.load offset=16
          i32.const 1
          i32.ne
          br_if 1 (;@2;)
        end
        local.get 0
        i32.load
        local.set 0
        local.get 2
        i32.const 0
        i32.store offset=12
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.const 128
                i32.lt_u
                br_if 0 (;@6;)
                local.get 0
                i32.const 2048
                i32.lt_u
                br_if 1 (;@5;)
                local.get 0
                i32.const 65536
                i32.ge_u
                br_if 2 (;@4;)
                local.get 2
                local.get 0
                i32.const 63
                i32.and
                i32.const 128
                i32.or
                i32.store8 offset=14
                local.get 2
                local.get 0
                i32.const 12
                i32.shr_u
                i32.const 224
                i32.or
                i32.store8 offset=12
                local.get 2
                local.get 0
                i32.const 6
                i32.shr_u
                i32.const 63
                i32.and
                i32.const 128
                i32.or
                i32.store8 offset=13
                i32.const 3
                local.set 0
                br 3 (;@3;)
              end
              local.get 2
              local.get 0
              i32.store8 offset=12
              i32.const 1
              local.set 0
              br 2 (;@3;)
            end
            local.get 2
            local.get 0
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            local.get 2
            local.get 0
            i32.const 6
            i32.shr_u
            i32.const 192
            i32.or
            i32.store8 offset=12
            i32.const 2
            local.set 0
            br 1 (;@3;)
          end
          local.get 2
          local.get 0
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=15
          local.get 2
          local.get 0
          i32.const 18
          i32.shr_u
          i32.const 240
          i32.or
          i32.store8 offset=12
          local.get 2
          local.get 0
          i32.const 6
          i32.shr_u
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=14
          local.get 2
          local.get 0
          i32.const 12
          i32.shr_u
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=13
          i32.const 4
          local.set 0
        end
        local.get 1
        local.get 2
        i32.const 12
        i32.add
        local.get 0
        call $_ZN4core3fmt9Formatter3pad17h48db8215dd23db50E
        local.set 1
        br 1 (;@1;)
      end
      local.get 1
      i32.load offset=24
      local.get 0
      i32.load
      local.get 1
      i32.const 28
      i32.add
      i32.load
      i32.load offset=16
      call_indirect (type 1)
      local.set 1
    end
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 1)
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in17h31198357b084008bE (type 2) (param i32 i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const -1
        i32.le_s
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            br_if 0 (;@4;)
            i32.const 1
            local.set 2
            br 1 (;@3;)
          end
          local.get 1
          i32.const 1
          call $__rust_alloc
          local.tee 2
          i32.eqz
          br_if 2 (;@1;)
        end
        local.get 0
        local.get 1
        i32.store offset=4
        local.get 0
        local.get 2
        i32.store
        return
      end
      call $_ZN5alloc7raw_vec17capacity_overflow17hb5aace515ee70317E
      unreachable
    end
    local.get 1
    i32.const 1
    call $_ZN5alloc5alloc18handle_alloc_error17h8e32739f1fa5c58cE
    unreachable)
  (func $_ZN4core3fmt5write17hbe0dd8e9c0d9aeb7E (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    i32.const 52
    i32.add
    local.get 1
    i32.store
    local.get 3
    i32.const 3
    i32.store8 offset=56
    local.get 3
    i64.const 137438953472
    i64.store offset=24
    local.get 3
    local.get 0
    i32.store offset=48
    i32.const 0
    local.set 4
    local.get 3
    i32.const 0
    i32.store offset=40
    local.get 3
    i32.const 0
    i32.store offset=32
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.load offset=8
            local.tee 5
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.load
            local.set 6
            local.get 2
            i32.load offset=4
            local.tee 7
            local.get 2
            i32.const 12
            i32.add
            i32.load
            i32.const 134217727
            i32.and
            local.tee 8
            local.get 8
            local.get 7
            i32.gt_u
            select
            local.tee 9
            i32.eqz
            br_if 1 (;@3;)
            local.get 9
            i32.const 3
            i32.shl
            i32.const -8
            i32.add
            local.set 10
            i32.const 0
            local.set 4
            loop  ;; label = @5
              local.get 0
              local.get 6
              local.get 4
              i32.add
              local.tee 8
              i32.load
              local.get 8
              i32.const 4
              i32.add
              i32.load
              local.get 1
              i32.load offset=12
              call_indirect (type 0)
              br_if 3 (;@2;)
              local.get 3
              local.get 5
              i32.const 28
              i32.add
              i32.load8_u
              i32.store8 offset=56
              local.get 3
              local.get 5
              i32.const 4
              i32.add
              i64.load align=4
              i64.const 32
              i64.rotl
              i64.store offset=24
              local.get 3
              i32.const 16
              i32.add
              local.get 2
              i32.load offset=16
              local.tee 0
              local.get 2
              i32.load offset=20
              local.tee 1
              local.get 5
              i32.const 20
              i32.add
              call $_ZN4core3fmt8getcount17h94e80423a9eced8fE
              local.get 3
              local.get 3
              i64.load offset=16
              i64.store offset=32
              local.get 3
              i32.const 8
              i32.add
              local.get 0
              local.get 1
              local.get 5
              i32.const 12
              i32.add
              call $_ZN4core3fmt8getcount17h94e80423a9eced8fE
              local.get 3
              local.get 3
              i64.load offset=8
              i64.store offset=40
              block  ;; label = @6
                block  ;; label = @7
                  local.get 5
                  i32.load
                  local.tee 8
                  local.get 1
                  i32.ge_u
                  br_if 0 (;@7;)
                  local.get 0
                  local.get 8
                  i32.const 3
                  i32.shl
                  i32.add
                  local.tee 0
                  i32.load
                  local.get 3
                  i32.const 24
                  i32.add
                  local.get 0
                  i32.load offset=4
                  call_indirect (type 1)
                  br_if 5 (;@2;)
                  local.get 10
                  local.get 4
                  i32.ne
                  br_if 1 (;@6;)
                  local.get 9
                  local.set 4
                  br 4 (;@3;)
                end
                local.get 8
                local.get 1
                i32.const 13900
                call $_ZN4core9panicking18panic_bounds_check17hc53ca9df0b8509fdE
                unreachable
              end
              local.get 5
              i32.const 32
              i32.add
              local.set 5
              local.get 4
              i32.const 8
              i32.add
              local.set 4
              local.get 3
              i32.load offset=52
              local.set 1
              local.get 3
              i32.load offset=48
              local.set 0
              br 0 (;@5;)
            end
          end
          local.get 2
          i32.load
          local.set 6
          local.get 2
          i32.load offset=4
          local.tee 7
          local.get 2
          i32.const 20
          i32.add
          i32.load
          local.tee 5
          local.get 5
          local.get 7
          i32.gt_u
          select
          local.tee 10
          i32.eqz
          br_if 0 (;@3;)
          local.get 10
          i32.const -1
          i32.add
          local.set 4
          local.get 2
          i32.load offset=16
          local.set 8
          i32.const 0
          local.set 5
          loop  ;; label = @4
            local.get 0
            local.get 6
            local.get 5
            i32.add
            local.tee 2
            i32.load
            local.get 2
            i32.const 4
            i32.add
            i32.load
            local.get 1
            i32.load offset=12
            call_indirect (type 0)
            br_if 2 (;@2;)
            local.get 8
            local.get 5
            i32.add
            local.tee 0
            i32.load
            local.get 3
            i32.const 24
            i32.add
            local.get 0
            i32.const 4
            i32.add
            i32.load
            call_indirect (type 1)
            br_if 2 (;@2;)
            block  ;; label = @5
              local.get 4
              br_if 0 (;@5;)
              local.get 10
              local.set 4
              br 2 (;@3;)
            end
            local.get 4
            i32.const -1
            i32.add
            local.set 4
            local.get 5
            i32.const 8
            i32.add
            local.set 5
            local.get 3
            i32.load offset=52
            local.set 1
            local.get 3
            i32.load offset=48
            local.set 0
            br 0 (;@4;)
          end
        end
        block  ;; label = @3
          local.get 7
          local.get 4
          i32.le_u
          br_if 0 (;@3;)
          local.get 3
          i32.load offset=48
          local.get 6
          local.get 4
          i32.const 3
          i32.shl
          i32.add
          local.tee 5
          i32.load
          local.get 5
          i32.load offset=4
          local.get 3
          i32.load offset=52
          i32.load offset=12
          call_indirect (type 0)
          br_if 1 (;@2;)
        end
        i32.const 0
        local.set 5
        br 1 (;@1;)
      end
      i32.const 1
      local.set 5
    end
    local.get 3
    i32.const 64
    i32.add
    global.set 0
    local.get 5)
  (func $__rust_dealloc (type 2) (param i32 i32)
    local.get 0
    local.get 1
    call $__rg_dealloc)
  (func $_ZN4core6result13unwrap_failed17hfa73799f0c5a0f1cE (type 7) (param i32 i32 i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 5
    global.set 0
    local.get 5
    local.get 1
    i32.store offset=12
    local.get 5
    local.get 0
    i32.store offset=8
    local.get 5
    local.get 3
    i32.store offset=20
    local.get 5
    local.get 2
    i32.store offset=16
    local.get 5
    i32.const 44
    i32.add
    i32.const 2
    i32.store
    local.get 5
    i32.const 60
    i32.add
    i32.const 3
    i32.store
    local.get 5
    i64.const 2
    i64.store offset=28 align=4
    local.get 5
    i32.const 11256
    i32.store offset=24
    local.get 5
    i32.const 4
    i32.store offset=52
    local.get 5
    local.get 5
    i32.const 48
    i32.add
    i32.store offset=40
    local.get 5
    local.get 5
    i32.const 16
    i32.add
    i32.store offset=56
    local.get 5
    local.get 5
    i32.const 8
    i32.add
    i32.store offset=48
    local.get 5
    i32.const 24
    i32.add
    local.get 4
    call $_ZN4core9panicking9panic_fmt17hdf8d0243e0d4fcebE
    unreachable)
  (func $__rust_realloc (type 3) (param i32 i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    call $__rg_realloc)
  (func $__rust_alloc_error_handler (type 2) (param i32 i32)
    local.get 0
    local.get 1
    call $__rg_oom
    unreachable)
  (func $__rg_oom (type 2) (param i32 i32)
    (local i32)
    local.get 0
    local.get 1
    i32.const 0
    i32.load offset=15324
    local.tee 2
    i32.const 5
    local.get 2
    select
    call_indirect (type 2)
    unreachable
    unreachable)
  (func $_ZN4core9panicking5panic17h1f31dfdebac1729fE (type 8) (param i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    i32.const 20
    i32.add
    i32.const 0
    i32.store
    local.get 3
    i32.const 15240
    i32.store offset=16
    local.get 3
    i64.const 1
    i64.store offset=4 align=4
    local.get 3
    local.get 1
    i32.store offset=28
    local.get 3
    local.get 0
    i32.store offset=24
    local.get 3
    local.get 3
    i32.const 24
    i32.add
    i32.store
    local.get 3
    local.get 2
    call $_ZN4core9panicking9panic_fmt17hdf8d0243e0d4fcebE
    unreachable)
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve17h14e2bbdcfb5c0d23E (type 8) (param i32 i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.load offset=4
          local.tee 4
          local.get 1
          i32.sub
          local.get 2
          i32.ge_u
          br_if 0 (;@3;)
          local.get 1
          local.get 2
          i32.add
          local.tee 2
          local.get 1
          i32.lt_u
          br_if 2 (;@1;)
          local.get 4
          i32.const 1
          i32.shl
          local.tee 1
          local.get 2
          local.get 1
          local.get 2
          i32.gt_u
          select
          local.tee 1
          i32.const 8
          local.get 1
          i32.const 8
          i32.gt_u
          select
          local.set 1
          block  ;; label = @4
            block  ;; label = @5
              local.get 4
              i32.eqz
              br_if 0 (;@5;)
              local.get 3
              i32.const 16
              i32.add
              i32.const 8
              i32.add
              i32.const 1
              i32.store
              local.get 3
              local.get 4
              i32.store offset=20
              local.get 3
              local.get 0
              i32.load
              i32.store offset=16
              br 1 (;@4;)
            end
            local.get 3
            i32.const 0
            i32.store offset=16
          end
          local.get 3
          local.get 1
          i32.const 1
          local.get 3
          i32.const 16
          i32.add
          call $_ZN5alloc7raw_vec11finish_grow17h3e91ad6198eee436E
          local.get 3
          i32.const 8
          i32.add
          i32.load
          local.set 1
          local.get 3
          i32.load offset=4
          local.set 2
          local.get 3
          i32.load
          i32.const 1
          i32.eq
          br_if 1 (;@2;)
          local.get 0
          local.get 1
          i32.store offset=4
          local.get 0
          local.get 2
          i32.store
        end
        local.get 3
        i32.const 32
        i32.add
        global.set 0
        return
      end
      local.get 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      local.get 1
      call $_ZN5alloc5alloc18handle_alloc_error17h8e32739f1fa5c58cE
      unreachable
    end
    call $_ZN5alloc7raw_vec17capacity_overflow17hb5aace515ee70317E
    unreachable)
  (func $_ZN115_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..SpecExtend$LT$$RF$T$C$core..slice..iter..Iter$LT$T$GT$$GT$$GT$11spec_extend17haaef633926038d87E (type 8) (param i32 i32 i32)
    local.get 0
    local.get 0
    i32.load offset=8
    local.get 2
    local.get 1
    i32.sub
    local.tee 2
    call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve17h14e2bbdcfb5c0d23E
    local.get 0
    i32.load
    local.get 0
    i32.load offset=8
    i32.add
    local.get 1
    local.get 2
    call $memcpy
    drop
    local.get 0
    local.get 0
    i32.load offset=8
    local.get 2
    i32.add
    i32.store offset=8)
  (func $_ZN4core3ptr13drop_in_place17h1dffba9b32db7ea5E (type 9) (param i32))
  (func $_ZN3std5alloc24default_alloc_error_hook17hb03d50b55ebca620E (type 2) (param i32 i32))
  (func $_ZN4core3ptr13drop_in_place17h5842ab0252ffe1ccE (type 9) (param i32))
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h29e73825d1f5f372E (type 0) (param i32 i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    local.get 1
    local.get 2
    i32.add
    call $_ZN115_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..SpecExtend$LT$$RF$T$C$core..slice..iter..Iter$LT$T$GT$$GT$$GT$11spec_extend17haaef633926038d87E
    i32.const 0)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h9cff9d77aed7896dE (type 1) (param i32 i32) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 0
    i32.load
    local.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 128
            i32.lt_u
            br_if 0 (;@4;)
            local.get 2
            i32.const 0
            i32.store offset=12
            local.get 1
            i32.const 2048
            i32.lt_u
            br_if 1 (;@3;)
            block  ;; label = @5
              local.get 1
              i32.const 65536
              i32.ge_u
              br_if 0 (;@5;)
              local.get 2
              local.get 1
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=14
              local.get 2
              local.get 1
              i32.const 12
              i32.shr_u
              i32.const 224
              i32.or
              i32.store8 offset=12
              local.get 2
              local.get 1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=13
              i32.const 3
              local.set 1
              br 3 (;@2;)
            end
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=15
            local.get 2
            local.get 1
            i32.const 18
            i32.shr_u
            i32.const 240
            i32.or
            i32.store8 offset=12
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            i32.const 4
            local.set 1
            br 2 (;@2;)
          end
          block  ;; label = @4
            local.get 0
            i32.load offset=8
            local.tee 3
            local.get 0
            i32.load offset=4
            i32.ne
            br_if 0 (;@4;)
            local.get 0
            local.get 3
            i32.const 1
            call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve17h14e2bbdcfb5c0d23E
            local.get 0
            i32.load offset=8
            local.set 3
          end
          local.get 0
          i32.load
          local.get 3
          i32.add
          local.get 1
          i32.store8
          local.get 0
          local.get 0
          i32.load offset=8
          i32.const 1
          i32.add
          i32.store offset=8
          br 2 (;@1;)
        end
        local.get 2
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 192
        i32.or
        i32.store8 offset=12
        i32.const 2
        local.set 1
      end
      local.get 0
      local.get 2
      i32.const 12
      i32.add
      local.get 2
      i32.const 12
      i32.add
      local.get 1
      i32.add
      call $_ZN115_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..SpecExtend$LT$$RF$T$C$core..slice..iter..Iter$LT$T$GT$$GT$$GT$11spec_extend17haaef633926038d87E
    end
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    i32.const 0)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17hf1fbebbdcf92fdc7E (type 1) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 0
    i32.load
    i32.store offset=4
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    i32.const 4
    i32.add
    i32.const 10736
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17hbe0dd8e9c0d9aeb7E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set 0
    local.get 1)
  (func $_ZN4core3fmt3num55_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$usize$GT$3fmt17hf6663d383c87beb5E (type 1) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 128
    i32.sub
    local.tee 2
    global.set 0
    i32.const 0
    local.set 3
    loop  ;; label = @1
      local.get 2
      local.get 3
      i32.add
      i32.const 127
      i32.add
      local.get 0
      i32.const 15
      i32.and
      local.tee 4
      i32.const 48
      i32.or
      local.get 4
      i32.const 87
      i32.add
      local.get 4
      i32.const 10
      i32.lt_u
      select
      i32.store8
      local.get 3
      i32.const -1
      i32.add
      local.set 3
      local.get 0
      i32.const 4
      i32.shr_u
      local.tee 0
      br_if 0 (;@1;)
    end
    block  ;; label = @1
      local.get 3
      i32.const 128
      i32.add
      local.tee 0
      i32.const 129
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      i32.const 128
      i32.const 10960
      call $_ZN4core5slice5index26slice_start_index_len_fail17h096dafa4b2741edeE
      unreachable
    end
    local.get 1
    i32.const 1
    i32.const 10976
    i32.const 2
    local.get 2
    local.get 3
    i32.add
    i32.const 128
    i32.add
    i32.const 0
    local.get 3
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17h07aef0d894bb255dE
    local.set 3
    local.get 2
    i32.const 128
    i32.add
    global.set 0
    local.get 3)
  (func $_ZN4core5slice5index26slice_start_index_len_fail17h096dafa4b2741edeE (type 8) (param i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    local.get 1
    i32.store offset=4
    local.get 3
    local.get 0
    i32.store
    local.get 3
    i32.const 28
    i32.add
    i32.const 2
    i32.store
    local.get 3
    i32.const 44
    i32.add
    i32.const 6
    i32.store
    local.get 3
    i64.const 2
    i64.store offset=12 align=4
    local.get 3
    i32.const 11212
    i32.store offset=8
    local.get 3
    i32.const 6
    i32.store offset=36
    local.get 3
    local.get 3
    i32.const 32
    i32.add
    i32.store offset=24
    local.get 3
    local.get 3
    i32.const 4
    i32.add
    i32.store offset=40
    local.get 3
    local.get 3
    i32.store offset=32
    local.get 3
    i32.const 8
    i32.add
    local.get 2
    call $_ZN4core9panicking9panic_fmt17hdf8d0243e0d4fcebE
    unreachable)
  (func $_ZN4core3fmt9Formatter12pad_integral17h07aef0d894bb255dE (type 10) (param i32 i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 6
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.eqz
        br_if 0 (;@2;)
        i32.const 43
        i32.const 1114112
        local.get 0
        i32.load
        local.tee 7
        i32.const 1
        i32.and
        local.tee 1
        select
        local.set 8
        local.get 1
        local.get 5
        i32.add
        local.set 9
        br 1 (;@1;)
      end
      local.get 5
      i32.const 1
      i32.add
      local.set 9
      local.get 0
      i32.load
      local.set 7
      i32.const 45
      local.set 8
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 7
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        i32.const 0
        local.set 2
        br 1 (;@1;)
      end
      i32.const 0
      local.set 10
      block  ;; label = @2
        local.get 3
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        local.set 11
        local.get 2
        local.set 1
        loop  ;; label = @3
          local.get 10
          local.get 1
          i32.load8_u
          i32.const 192
          i32.and
          i32.const 128
          i32.eq
          i32.add
          local.set 10
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 11
          i32.const -1
          i32.add
          local.tee 11
          br_if 0 (;@3;)
        end
      end
      local.get 9
      local.get 3
      i32.add
      local.get 10
      i32.sub
      local.set 9
    end
    i32.const 1
    local.set 1
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load offset=8
        i32.const 1
        i32.eq
        br_if 0 (;@2;)
        local.get 0
        local.get 8
        local.get 2
        local.get 3
        call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17hf48b2e0a8b365e61E
        br_if 1 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 4
        local.get 5
        local.get 0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 0)
        local.set 1
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 0
        i32.const 12
        i32.add
        i32.load
        local.tee 10
        local.get 9
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        local.get 8
        local.get 2
        local.get 3
        call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17hf48b2e0a8b365e61E
        br_if 1 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 4
        local.get 5
        local.get 0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 0)
        local.set 1
        br 1 (;@1;)
      end
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 7
            i32.const 8
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            i32.load offset=4
            local.set 7
            local.get 0
            i32.const 48
            i32.store offset=4
            local.get 0
            i32.load8_u offset=32
            local.set 12
            i32.const 1
            local.set 1
            local.get 0
            i32.const 1
            i32.store8 offset=32
            local.get 0
            local.get 8
            local.get 2
            local.get 3
            call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17hf48b2e0a8b365e61E
            br_if 3 (;@1;)
            i32.const 1
            local.set 1
            local.get 6
            i32.const 8
            i32.add
            local.get 0
            local.get 10
            local.get 9
            i32.sub
            i32.const 1
            call $_ZN4core3fmt9Formatter7padding17h7a1896b194aaa2f4E
            local.get 6
            i32.load offset=8
            local.tee 11
            i32.const 1114112
            i32.eq
            br_if 3 (;@1;)
            local.get 6
            i32.load offset=12
            local.set 10
            local.get 0
            i32.load offset=24
            local.get 4
            local.get 5
            local.get 0
            i32.const 28
            i32.add
            i32.load
            i32.load offset=12
            call_indirect (type 0)
            br_if 3 (;@1;)
            local.get 10
            i32.const 1
            i32.add
            local.set 10
            local.get 0
            i32.load offset=28
            local.set 3
            local.get 0
            i32.load offset=24
            local.set 2
            loop  ;; label = @5
              local.get 10
              i32.const -1
              i32.add
              local.tee 10
              i32.eqz
              br_if 2 (;@3;)
              i32.const 1
              local.set 1
              local.get 2
              local.get 11
              local.get 3
              i32.load offset=16
              call_indirect (type 1)
              i32.eqz
              br_if 0 (;@5;)
              br 4 (;@1;)
            end
          end
          i32.const 1
          local.set 1
          local.get 6
          local.get 0
          local.get 10
          local.get 9
          i32.sub
          i32.const 1
          call $_ZN4core3fmt9Formatter7padding17h7a1896b194aaa2f4E
          local.get 6
          i32.load
          local.tee 11
          i32.const 1114112
          i32.eq
          br_if 2 (;@1;)
          local.get 6
          i32.load offset=4
          local.set 10
          local.get 0
          local.get 8
          local.get 2
          local.get 3
          call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17hf48b2e0a8b365e61E
          br_if 2 (;@1;)
          local.get 0
          i32.load offset=24
          local.get 4
          local.get 5
          local.get 0
          i32.const 28
          i32.add
          i32.load
          i32.load offset=12
          call_indirect (type 0)
          br_if 2 (;@1;)
          local.get 10
          i32.const 1
          i32.add
          local.set 10
          local.get 0
          i32.load offset=28
          local.set 3
          local.get 0
          i32.load offset=24
          local.set 0
          loop  ;; label = @4
            local.get 10
            i32.const -1
            i32.add
            local.tee 10
            i32.eqz
            br_if 2 (;@2;)
            i32.const 1
            local.set 1
            local.get 0
            local.get 11
            local.get 3
            i32.load offset=16
            call_indirect (type 1)
            i32.eqz
            br_if 0 (;@4;)
            br 3 (;@1;)
          end
        end
        local.get 0
        local.get 12
        i32.store8 offset=32
        local.get 0
        local.get 7
        i32.store offset=4
      end
      i32.const 0
      local.set 1
    end
    local.get 6
    i32.const 16
    i32.add
    global.set 0
    local.get 1)
  (func $_ZN4core3fmt3num55_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$usize$GT$3fmt17ha0f7aebd5326c295E (type 1) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 128
    i32.sub
    local.tee 2
    global.set 0
    i32.const 0
    local.set 3
    loop  ;; label = @1
      local.get 2
      local.get 3
      i32.add
      i32.const 127
      i32.add
      local.get 0
      i32.const 15
      i32.and
      local.tee 4
      i32.const 48
      i32.or
      local.get 4
      i32.const 55
      i32.add
      local.get 4
      i32.const 10
      i32.lt_u
      select
      i32.store8
      local.get 3
      i32.const -1
      i32.add
      local.set 3
      local.get 0
      i32.const 4
      i32.shr_u
      local.tee 0
      br_if 0 (;@1;)
    end
    block  ;; label = @1
      local.get 3
      i32.const 128
      i32.add
      local.tee 0
      i32.const 129
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      i32.const 128
      i32.const 10960
      call $_ZN4core5slice5index26slice_start_index_len_fail17h096dafa4b2741edeE
      unreachable
    end
    local.get 1
    i32.const 1
    i32.const 10976
    i32.const 2
    local.get 2
    local.get 3
    i32.add
    i32.const 128
    i32.add
    i32.const 0
    local.get 3
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17h07aef0d894bb255dE
    local.set 3
    local.get 2
    i32.const 128
    i32.add
    global.set 0
    local.get 3)
  (func $_ZN4core5slice5index24slice_end_index_len_fail17h39f0c0387bdaee32E (type 8) (param i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    local.get 1
    i32.store offset=4
    local.get 3
    local.get 0
    i32.store
    local.get 3
    i32.const 28
    i32.add
    i32.const 2
    i32.store
    local.get 3
    i32.const 44
    i32.add
    i32.const 6
    i32.store
    local.get 3
    i64.const 2
    i64.store offset=12 align=4
    local.get 3
    i32.const 11092
    i32.store offset=8
    local.get 3
    i32.const 6
    i32.store offset=36
    local.get 3
    local.get 3
    i32.const 32
    i32.add
    i32.store offset=24
    local.get 3
    local.get 3
    i32.const 4
    i32.add
    i32.store offset=40
    local.get 3
    local.get 3
    i32.store offset=32
    local.get 3
    i32.const 8
    i32.add
    local.get 2
    call $_ZN4core9panicking9panic_fmt17hdf8d0243e0d4fcebE
    unreachable)
  (func $_ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$usize$GT$3fmt17h60d343ae9ad25d2dE (type 1) (param i32 i32) (result i32)
    local.get 0
    i64.load32_u
    i32.const 1
    local.get 1
    call $_ZN4core3fmt3num3imp7fmt_u6417h53be2cf8685583ddE)
  (func $_ZN4core9panicking9panic_fmt17hdf8d0243e0d4fcebE (type 2) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 1
    i32.store offset=12
    local.get 2
    local.get 0
    i32.store offset=8
    local.get 2
    i32.const 14076
    i32.store offset=4
    local.get 2
    i32.const 15240
    i32.store
    local.get 2
    call $rust_begin_unwind
    unreachable)
  (func $_ZN4core5slice5index22slice_index_order_fail17h3b506dc87657b590E (type 8) (param i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    local.get 1
    i32.store offset=4
    local.get 3
    local.get 0
    i32.store
    local.get 3
    i32.const 28
    i32.add
    i32.const 2
    i32.store
    local.get 3
    i32.const 44
    i32.add
    i32.const 6
    i32.store
    local.get 3
    i64.const 2
    i64.store offset=12 align=4
    local.get 3
    i32.const 11160
    i32.store offset=8
    local.get 3
    i32.const 6
    i32.store offset=36
    local.get 3
    local.get 3
    i32.const 32
    i32.add
    i32.store offset=24
    local.get 3
    local.get 3
    i32.const 4
    i32.add
    i32.store offset=40
    local.get 3
    local.get 3
    i32.store offset=32
    local.get 3
    i32.const 8
    i32.add
    local.get 2
    call $_ZN4core9panicking9panic_fmt17hdf8d0243e0d4fcebE
    unreachable)
  (func $_ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h768de56d2684a6efE (type 1) (param i32 i32) (result i32)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.load
          local.tee 3
          i32.const 16
          i32.and
          br_if 0 (;@3;)
          local.get 0
          i32.load
          local.set 4
          block  ;; label = @4
            local.get 3
            i32.const 32
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            i32.const 1
            local.set 3
            local.get 4
            local.get 1
            call $_ZN4core3fmt3num55_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$usize$GT$3fmt17ha0f7aebd5326c295E
            i32.eqz
            br_if 2 (;@2;)
            br 3 (;@1;)
          end
          i32.const 1
          local.set 3
          local.get 4
          i64.extend_i32_u
          i32.const 1
          local.get 1
          call $_ZN4core3fmt3num3imp7fmt_u6417h53be2cf8685583ddE
          br_if 2 (;@1;)
          br 1 (;@2;)
        end
        i32.const 1
        local.set 3
        local.get 0
        i32.load
        local.get 1
        call $_ZN4core3fmt3num55_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$usize$GT$3fmt17hf6663d383c87beb5E
        br_if 1 (;@1;)
      end
      local.get 1
      i32.const 28
      i32.add
      i32.load
      local.set 4
      local.get 1
      i32.load offset=24
      local.set 5
      local.get 2
      i32.const 28
      i32.add
      i32.const 0
      i32.store
      local.get 2
      i32.const 15240
      i32.store offset=24
      local.get 2
      i64.const 1
      i64.store offset=12 align=4
      local.get 2
      i32.const 11248
      i32.store offset=8
      i32.const 1
      local.set 3
      local.get 5
      local.get 4
      local.get 2
      i32.const 8
      i32.add
      call $_ZN4core3fmt5write17hbe0dd8e9c0d9aeb7E
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.load
          local.tee 3
          i32.const 16
          i32.and
          br_if 0 (;@3;)
          local.get 0
          i32.load offset=4
          local.set 0
          block  ;; label = @4
            local.get 3
            i32.const 32
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            i32.const 0
            local.set 3
            local.get 0
            local.get 1
            call $_ZN4core3fmt3num55_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$usize$GT$3fmt17ha0f7aebd5326c295E
            br_if 2 (;@2;)
            br 3 (;@1;)
          end
          i32.const 0
          local.set 3
          local.get 0
          i64.extend_i32_u
          i32.const 1
          local.get 1
          call $_ZN4core3fmt3num3imp7fmt_u6417h53be2cf8685583ddE
          i32.eqz
          br_if 2 (;@1;)
          br 1 (;@2;)
        end
        i32.const 0
        local.set 3
        local.get 0
        i32.load offset=4
        local.get 1
        call $_ZN4core3fmt3num55_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$usize$GT$3fmt17hf6663d383c87beb5E
        i32.eqz
        br_if 1 (;@1;)
      end
      i32.const 1
      local.set 3
    end
    local.get 2
    i32.const 32
    i32.add
    global.set 0
    local.get 3)
  (func $_ZN4core3fmt3num3imp7fmt_u6417h53be2cf8685583ddE (type 11) (param i64 i32 i32) (result i32)
    (local i32 i32 i64 i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 3
    global.set 0
    i32.const 39
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i64.const 10000
        i64.ge_u
        br_if 0 (;@2;)
        local.get 0
        local.set 5
        br 1 (;@1;)
      end
      i32.const 39
      local.set 4
      loop  ;; label = @2
        local.get 3
        i32.const 9
        i32.add
        local.get 4
        i32.add
        local.tee 6
        i32.const -2
        i32.add
        local.get 0
        i64.const 10000
        i64.rem_u
        i32.wrap_i64
        local.tee 7
        i32.const 100
        i32.rem_u
        i32.const 1
        i32.shl
        i32.const 10760
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        local.get 6
        i32.const -4
        i32.add
        local.get 7
        i32.const 100
        i32.div_u
        i32.const 1
        i32.shl
        i32.const 10760
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        local.get 4
        i32.const -4
        i32.add
        local.set 4
        local.get 0
        i64.const 99999999
        i64.gt_u
        local.set 6
        local.get 0
        i64.const 10000
        i64.div_u
        local.tee 5
        local.set 0
        local.get 6
        br_if 0 (;@2;)
      end
    end
    block  ;; label = @1
      local.get 5
      i32.wrap_i64
      local.tee 6
      i32.const 99
      i32.le_s
      br_if 0 (;@1;)
      local.get 3
      i32.const 9
      i32.add
      local.get 4
      i32.const -2
      i32.add
      local.tee 4
      i32.add
      local.get 5
      i32.wrap_i64
      i32.const 65535
      i32.and
      local.tee 6
      i32.const 100
      i32.rem_u
      i32.const 1
      i32.shl
      i32.const 10760
      i32.add
      i32.load16_u align=1
      i32.store16 align=1
      local.get 6
      i32.const 100
      i32.div_u
      local.set 6
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 6
        i32.const 10
        i32.lt_s
        br_if 0 (;@2;)
        local.get 3
        i32.const 9
        i32.add
        local.get 4
        i32.const -2
        i32.add
        local.tee 4
        i32.add
        local.get 6
        i32.const 1
        i32.shl
        i32.const 10760
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        br 1 (;@1;)
      end
      local.get 3
      i32.const 9
      i32.add
      local.get 4
      i32.const -1
      i32.add
      local.tee 4
      i32.add
      local.get 6
      i32.const 48
      i32.add
      i32.store8
    end
    local.get 2
    local.get 1
    i32.const 15240
    i32.const 0
    local.get 3
    i32.const 9
    i32.add
    local.get 4
    i32.add
    i32.const 39
    local.get 4
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17h07aef0d894bb255dE
    local.set 4
    local.get 3
    i32.const 48
    i32.add
    global.set 0
    local.get 4)
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h4650a8e2c1d42a98E (type 12) (param i32) (result i64)
    i64.const 2773228358778157168)
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hfcaccb2e0e81eaebE (type 1) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    local.get 0
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 1))
  (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h31e3a003075045c9E (type 1) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    call $_ZN4core3fmt9Formatter3pad17h48db8215dd23db50E)
  (func $_ZN4core5slice6memchr19memchr_general_case17h5889cd0f1abb24d8E (type 5) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32)
    i32.const 0
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              i32.const 0
              local.get 2
              i32.sub
              i32.const 3
              i32.and
              local.tee 5
              i32.eqz
              br_if 0 (;@5;)
              local.get 3
              local.get 5
              local.get 5
              local.get 3
              i32.gt_u
              select
              local.set 5
              local.get 1
              i32.const 255
              i32.and
              local.set 6
              block  ;; label = @6
                loop  ;; label = @7
                  local.get 2
                  local.get 4
                  i32.add
                  i32.load8_u
                  local.get 6
                  i32.eq
                  br_if 1 (;@6;)
                  local.get 5
                  local.get 4
                  i32.const 1
                  i32.add
                  local.tee 4
                  i32.ne
                  br_if 0 (;@7;)
                end
                local.get 5
                local.get 3
                i32.const -8
                i32.add
                local.tee 7
                i32.le_u
                br_if 2 (;@4;)
                br 3 (;@3;)
              end
              i32.const 1
              local.set 8
              br 3 (;@2;)
            end
            local.get 3
            i32.const -8
            i32.add
            local.set 7
            i32.const 0
            local.set 5
          end
          local.get 1
          i32.const 255
          i32.and
          i32.const 16843009
          i32.mul
          local.set 4
          block  ;; label = @4
            loop  ;; label = @5
              local.get 2
              local.get 5
              i32.add
              local.tee 6
              i32.const 4
              i32.add
              i32.load
              local.get 4
              i32.xor
              local.tee 8
              i32.const -1
              i32.xor
              local.get 8
              i32.const -16843009
              i32.add
              i32.and
              local.get 6
              i32.load
              local.get 4
              i32.xor
              local.tee 6
              i32.const -1
              i32.xor
              local.get 6
              i32.const -16843009
              i32.add
              i32.and
              i32.or
              i32.const -2139062144
              i32.and
              br_if 1 (;@4;)
              local.get 5
              i32.const 8
              i32.add
              local.tee 5
              local.get 7
              i32.le_u
              br_if 0 (;@5;)
            end
          end
          local.get 5
          local.get 3
          i32.gt_u
          br_if 2 (;@1;)
        end
        i32.const 0
        local.set 6
        i32.const 0
        local.set 8
        block  ;; label = @3
          local.get 5
          local.get 3
          i32.eq
          br_if 0 (;@3;)
          local.get 2
          local.get 5
          i32.add
          local.set 2
          local.get 3
          local.get 5
          i32.sub
          local.set 6
          i32.const 0
          local.set 4
          local.get 1
          i32.const 255
          i32.and
          local.set 8
          block  ;; label = @4
            loop  ;; label = @5
              local.get 2
              local.get 4
              i32.add
              i32.load8_u
              local.get 8
              i32.eq
              br_if 1 (;@4;)
              local.get 6
              local.get 4
              i32.const 1
              i32.add
              local.tee 4
              i32.ne
              br_if 0 (;@5;)
            end
            i32.const 0
            local.set 8
            br 1 (;@3;)
          end
          i32.const 1
          local.set 8
          local.get 4
          local.set 6
        end
        local.get 6
        local.get 5
        i32.add
        local.set 4
      end
      local.get 0
      local.get 4
      i32.store offset=4
      local.get 0
      local.get 8
      i32.store
      return
    end
    local.get 5
    local.get 3
    i32.const 11272
    call $_ZN4core5slice5index26slice_start_index_len_fail17h096dafa4b2741edeE
    unreachable)
  (func $_ZN4core7unicode9printable12is_printable17hdf644931c213b146E (type 4) (param i32) (result i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.const 65536
          i32.lt_u
          br_if 0 (;@3;)
          local.get 0
          i32.const 131072
          i32.lt_u
          br_if 2 (;@1;)
          i32.const 0
          local.set 1
          local.get 0
          i32.const -201547
          i32.add
          i32.const 716213
          i32.lt_u
          br_if 1 (;@2;)
          local.get 0
          i32.const -195102
          i32.add
          i32.const 1506
          i32.lt_u
          br_if 1 (;@2;)
          local.get 0
          i32.const -191457
          i32.add
          i32.const 3103
          i32.lt_u
          br_if 1 (;@2;)
          local.get 0
          i32.const -183970
          i32.add
          i32.const 14
          i32.lt_u
          br_if 1 (;@2;)
          local.get 0
          i32.const 2097150
          i32.and
          i32.const 178206
          i32.eq
          br_if 1 (;@2;)
          local.get 0
          i32.const -173790
          i32.add
          i32.const 34
          i32.lt_u
          br_if 1 (;@2;)
          local.get 0
          i32.const -177973
          i32.add
          i32.const 11
          i32.lt_u
          br_if 1 (;@2;)
          local.get 0
          i32.const 918000
          i32.lt_u
          return
        end
        local.get 0
        i32.const 11404
        i32.const 41
        i32.const 11486
        i32.const 290
        i32.const 11776
        i32.const 309
        call $_ZN4core7unicode9printable5check17h7652eba4a5e67530E
        local.set 1
      end
      local.get 1
      return
    end
    local.get 0
    i32.const 12085
    i32.const 38
    i32.const 12161
    i32.const 175
    i32.const 12336
    i32.const 419
    call $_ZN4core7unicode9printable5check17h7652eba4a5e67530E)
  (func $_ZN4core7unicode9printable5check17h7652eba4a5e67530E (type 13) (param i32 i32 i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    i32.const 1
    local.set 7
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              i32.eqz
              br_if 0 (;@5;)
              local.get 1
              local.get 2
              i32.const 1
              i32.shl
              i32.add
              local.set 8
              local.get 0
              i32.const 65280
              i32.and
              i32.const 8
              i32.shr_u
              local.set 9
              i32.const 0
              local.set 10
              local.get 0
              i32.const 255
              i32.and
              local.set 11
              loop  ;; label = @6
                local.get 1
                i32.const 2
                i32.add
                local.set 12
                local.get 10
                local.get 1
                i32.load8_u offset=1
                local.tee 2
                i32.add
                local.set 13
                block  ;; label = @7
                  local.get 1
                  i32.load8_u
                  local.tee 1
                  local.get 9
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 1
                  local.get 9
                  i32.gt_u
                  br_if 2 (;@5;)
                  local.get 13
                  local.set 10
                  local.get 12
                  local.set 1
                  local.get 12
                  local.get 8
                  i32.ne
                  br_if 1 (;@6;)
                  br 2 (;@5;)
                end
                local.get 13
                local.get 10
                i32.lt_u
                br_if 2 (;@4;)
                local.get 13
                local.get 4
                i32.gt_u
                br_if 3 (;@3;)
                local.get 3
                local.get 10
                i32.add
                local.set 1
                block  ;; label = @7
                  loop  ;; label = @8
                    local.get 2
                    i32.eqz
                    br_if 1 (;@7;)
                    local.get 2
                    i32.const -1
                    i32.add
                    local.set 2
                    local.get 1
                    i32.load8_u
                    local.set 10
                    local.get 1
                    i32.const 1
                    i32.add
                    local.set 1
                    local.get 10
                    local.get 11
                    i32.ne
                    br_if 0 (;@8;)
                  end
                  i32.const 0
                  local.set 7
                  br 6 (;@1;)
                end
                local.get 13
                local.set 10
                local.get 12
                local.set 1
                local.get 12
                local.get 8
                i32.ne
                br_if 0 (;@6;)
              end
            end
            local.get 6
            i32.eqz
            br_if 3 (;@1;)
            local.get 5
            local.get 6
            i32.add
            local.set 11
            local.get 0
            i32.const 65535
            i32.and
            local.set 10
            local.get 5
            i32.const 1
            i32.add
            local.set 2
            i32.const 1
            local.set 7
            loop  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 5
                  i32.load8_u
                  local.tee 1
                  i32.const 24
                  i32.shl
                  i32.const 24
                  i32.shr_s
                  local.tee 13
                  i32.const 0
                  i32.lt_s
                  br_if 0 (;@7;)
                  local.get 2
                  local.set 5
                  br 1 (;@6;)
                end
                local.get 2
                local.get 11
                i32.eq
                br_if 4 (;@2;)
                local.get 2
                i32.const 1
                i32.add
                local.set 5
                local.get 13
                i32.const 127
                i32.and
                i32.const 8
                i32.shl
                local.get 2
                i32.load8_u
                i32.or
                local.set 1
              end
              local.get 10
              local.get 1
              i32.sub
              local.tee 10
              i32.const 0
              i32.lt_s
              br_if 4 (;@1;)
              local.get 7
              i32.const 1
              i32.xor
              local.set 7
              local.get 5
              local.get 11
              i32.eq
              br_if 4 (;@1;)
              local.get 5
              i32.const 1
              i32.add
              local.set 2
              br 0 (;@5;)
            end
          end
          local.get 10
          local.get 13
          i32.const 12756
          call $_ZN4core5slice5index22slice_index_order_fail17h3b506dc87657b590E
          unreachable
        end
        local.get 13
        local.get 4
        i32.const 12756
        call $_ZN4core5slice5index24slice_end_index_len_fail17h39f0c0387bdaee32E
        unreachable
      end
      i32.const 15197
      i32.const 43
      i32.const 12772
      call $_ZN4core9panicking5panic17h1f31dfdebac1729fE
      unreachable
    end
    local.get 7
    i32.const 1
    i32.and)
  (func $_ZN4core7unicode12unicode_data15grapheme_extend6lookup17h9eaf4e302a998ac5E (type 4) (param i32) (result i32)
    (local i32 i32 i32 i32 i32)
    local.get 0
    i32.const 11
    i32.shl
    local.set 1
    i32.const 0
    local.set 2
    i32.const 31
    local.set 3
    loop  ;; label = @1
      local.get 2
      local.get 3
      i32.const 1
      i32.shr_u
      local.tee 4
      local.get 2
      i32.add
      local.tee 5
      local.get 5
      i32.const 2
      i32.shl
      i32.const 13084
      i32.add
      i32.load
      i32.const 11
      i32.shl
      local.get 1
      i32.gt_u
      select
      local.set 2
      local.get 3
      local.get 4
      i32.sub
      local.tee 3
      i32.const 1
      i32.gt_u
      br_if 0 (;@1;)
    end
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          local.get 2
          i32.const 2
          i32.shl
          i32.const 13084
          i32.add
          i32.load
          i32.const 11
          i32.shl
          local.tee 3
          local.get 1
          i32.lt_u
          i32.add
          local.get 3
          local.get 1
          i32.eq
          i32.add
          local.tee 2
          i32.const 30
          i32.gt_u
          br_if 0 (;@3;)
          i32.const 689
          local.set 3
          block  ;; label = @4
            local.get 2
            i32.const 30
            i32.eq
            br_if 0 (;@4;)
            local.get 2
            i32.const 2
            i32.shl
            i32.const 13088
            i32.add
            i32.load
            i32.const 21
            i32.shr_u
            local.set 3
          end
          i32.const 0
          local.set 4
          block  ;; label = @4
            local.get 2
            i32.const -1
            i32.add
            local.tee 5
            local.get 2
            i32.gt_u
            br_if 0 (;@4;)
            local.get 5
            i32.const 31
            i32.ge_u
            br_if 3 (;@1;)
            local.get 5
            i32.const 2
            i32.shl
            i32.const 13084
            i32.add
            i32.load
            i32.const 2097151
            i32.and
            local.set 4
          end
          block  ;; label = @4
            local.get 3
            local.get 2
            i32.const 2
            i32.shl
            i32.const 13084
            i32.add
            i32.load
            i32.const 21
            i32.shr_u
            local.tee 2
            i32.const -1
            i32.xor
            i32.add
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            local.get 4
            i32.sub
            local.set 4
            local.get 2
            i32.const 689
            local.get 2
            i32.const 689
            i32.gt_u
            select
            local.set 5
            local.get 3
            i32.const -1
            i32.add
            local.set 1
            i32.const 0
            local.set 3
            loop  ;; label = @5
              local.get 5
              local.get 2
              i32.eq
              br_if 3 (;@2;)
              local.get 3
              local.get 2
              i32.const 13208
              i32.add
              i32.load8_u
              i32.add
              local.tee 3
              local.get 4
              i32.gt_u
              br_if 1 (;@4;)
              local.get 1
              local.get 2
              i32.const 1
              i32.add
              local.tee 2
              i32.ne
              br_if 0 (;@5;)
            end
            local.get 1
            local.set 2
          end
          local.get 2
          i32.const 1
          i32.and
          return
        end
        local.get 2
        i32.const 31
        i32.const 12928
        call $_ZN4core9panicking18panic_bounds_check17hc53ca9df0b8509fdE
        unreachable
      end
      local.get 5
      i32.const 689
      i32.const 12944
      call $_ZN4core9panicking18panic_bounds_check17hc53ca9df0b8509fdE
      unreachable
    end
    local.get 5
    i32.const 31
    i32.const 12912
    call $_ZN4core9panicking18panic_bounds_check17hc53ca9df0b8509fdE
    unreachable)
  (func $_ZN4core9panicking18panic_bounds_check17hc53ca9df0b8509fdE (type 8) (param i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    local.get 1
    i32.store offset=4
    local.get 3
    local.get 0
    i32.store
    local.get 3
    i32.const 28
    i32.add
    i32.const 2
    i32.store
    local.get 3
    i32.const 44
    i32.add
    i32.const 6
    i32.store
    local.get 3
    i64.const 2
    i64.store offset=12 align=4
    local.get 3
    i32.const 14092
    i32.store offset=8
    local.get 3
    i32.const 6
    i32.store offset=36
    local.get 3
    local.get 3
    i32.const 32
    i32.add
    i32.store offset=24
    local.get 3
    local.get 3
    i32.store offset=40
    local.get 3
    local.get 3
    i32.const 4
    i32.add
    i32.store offset=32
    local.get 3
    i32.const 8
    i32.add
    local.get 2
    call $_ZN4core9panicking9panic_fmt17hdf8d0243e0d4fcebE
    unreachable)
  (func $_ZN4core3ops8function6FnOnce9call_once17h8a477fe68b043fe4E (type 1) (param i32 i32) (result i32)
    local.get 0
    i32.load
    drop
    loop (result i32)  ;; label = @1
      br 0 (;@1;)
    end)
  (func $_ZN4core3fmt8getcount17h94e80423a9eced8fE (type 5) (param i32 i32 i32 i32)
    (local i32 i32)
    i32.const 0
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 3
            i32.load
            br_table 0 (;@4;) 1 (;@3;) 3 (;@1;) 0 (;@4;)
          end
          local.get 3
          i32.load offset=4
          local.set 5
          br 1 (;@2;)
        end
        block  ;; label = @3
          local.get 3
          i32.load offset=4
          local.tee 3
          local.get 2
          i32.lt_u
          br_if 0 (;@3;)
          local.get 3
          local.get 2
          i32.const 14028
          call $_ZN4core9panicking18panic_bounds_check17hc53ca9df0b8509fdE
          unreachable
        end
        local.get 1
        local.get 3
        i32.const 3
        i32.shl
        i32.add
        local.tee 3
        i32.load offset=4
        i32.const 7
        i32.ne
        br_if 1 (;@1;)
        local.get 3
        i32.load
        i32.load
        local.set 5
      end
      i32.const 1
      local.set 4
    end
    local.get 0
    local.get 5
    i32.store offset=4
    local.get 0
    local.get 4
    i32.store)
  (func $_ZN4core3ptr13drop_in_place17h6d8597584bc4ba58E (type 9) (param i32))
  (func $_ZN4core3fmt9Formatter12pad_integral12write_prefix17hf48b2e0a8b365e61E (type 3) (param i32 i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 1114112
        i32.eq
        br_if 0 (;@2;)
        i32.const 1
        local.set 4
        local.get 0
        i32.load offset=24
        local.get 1
        local.get 0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=16
        call_indirect (type 1)
        br_if 1 (;@1;)
      end
      block  ;; label = @2
        local.get 2
        br_if 0 (;@2;)
        i32.const 0
        return
      end
      local.get 0
      i32.load offset=24
      local.get 2
      local.get 3
      local.get 0
      i32.const 28
      i32.add
      i32.load
      i32.load offset=12
      call_indirect (type 0)
      local.set 4
    end
    local.get 4)
  (func $_ZN4core3fmt9Formatter7padding17h7a1896b194aaa2f4E (type 5) (param i32 i32 i32 i32)
    (local i32 i32 i32)
    i32.const 0
    local.set 4
    local.get 2
    local.set 5
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 3
          local.get 1
          i32.load8_u offset=32
          local.tee 6
          local.get 6
          i32.const 3
          i32.eq
          select
          i32.const 255
          i32.and
          br_table 2 (;@1;) 1 (;@2;) 0 (;@3;) 1 (;@2;) 2 (;@1;)
        end
        local.get 2
        i32.const 1
        i32.shr_u
        local.set 4
        local.get 2
        i32.const 1
        i32.add
        i32.const 1
        i32.shr_u
        local.set 5
        br 1 (;@1;)
      end
      i32.const 0
      local.set 5
      local.get 2
      local.set 4
    end
    local.get 4
    i32.const 1
    i32.add
    local.set 2
    block  ;; label = @1
      loop  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.const -1
          i32.add
          local.tee 2
          br_if 0 (;@3;)
          local.get 1
          i32.load offset=4
          local.set 1
          br 2 (;@1;)
        end
        local.get 1
        i32.load offset=24
        local.get 1
        i32.load offset=4
        local.get 1
        i32.load offset=28
        i32.load offset=16
        call_indirect (type 1)
        i32.eqz
        br_if 0 (;@2;)
      end
      i32.const 1114112
      local.set 1
    end
    local.get 0
    local.get 5
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func $_ZN4core3fmt9Formatter3pad17h48db8215dd23db50E (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    local.get 0
    i32.load offset=16
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            i32.load offset=8
            local.tee 5
            i32.const 1
            i32.eq
            br_if 0 (;@4;)
            local.get 4
            br_if 1 (;@3;)
            local.get 0
            i32.load offset=24
            local.get 1
            local.get 2
            local.get 0
            i32.const 28
            i32.add
            i32.load
            i32.load offset=12
            call_indirect (type 0)
            local.set 4
            br 3 (;@1;)
          end
          local.get 4
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 1
        local.get 2
        i32.add
        local.set 6
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.const 20
              i32.add
              i32.load
              local.tee 7
              br_if 0 (;@5;)
              i32.const 0
              local.set 8
              local.get 1
              local.set 9
              br 1 (;@4;)
            end
            i32.const 0
            local.set 8
            local.get 1
            local.set 9
            loop  ;; label = @5
              local.get 6
              local.get 9
              local.tee 4
              i32.eq
              br_if 2 (;@3;)
              local.get 4
              i32.const 1
              i32.add
              local.set 9
              block  ;; label = @6
                local.get 4
                i32.load8_s
                local.tee 10
                i32.const -1
                i32.gt_s
                br_if 0 (;@6;)
                local.get 10
                i32.const 255
                i32.and
                local.set 10
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 9
                    local.get 6
                    i32.ne
                    br_if 0 (;@8;)
                    i32.const 0
                    local.set 11
                    local.get 6
                    local.set 12
                    br 1 (;@7;)
                  end
                  local.get 4
                  i32.load8_u offset=1
                  i32.const 63
                  i32.and
                  local.set 11
                  local.get 4
                  i32.const 2
                  i32.add
                  local.tee 9
                  local.set 12
                end
                local.get 10
                i32.const 224
                i32.lt_u
                br_if 0 (;@6;)
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 12
                    local.get 6
                    i32.ne
                    br_if 0 (;@8;)
                    i32.const 0
                    local.set 13
                    local.get 6
                    local.set 12
                    br 1 (;@7;)
                  end
                  local.get 12
                  i32.load8_u
                  i32.const 63
                  i32.and
                  local.set 13
                  local.get 12
                  i32.const 1
                  i32.add
                  local.tee 9
                  local.set 12
                end
                local.get 10
                i32.const 240
                i32.lt_u
                br_if 0 (;@6;)
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 12
                    local.get 6
                    i32.ne
                    br_if 0 (;@8;)
                    i32.const 0
                    local.set 12
                    br 1 (;@7;)
                  end
                  local.get 12
                  i32.const 1
                  i32.add
                  local.set 9
                  local.get 12
                  i32.load8_u
                  i32.const 63
                  i32.and
                  local.set 12
                end
                local.get 11
                i32.const 12
                i32.shl
                local.get 10
                i32.const 18
                i32.shl
                i32.const 1835008
                i32.and
                i32.or
                local.get 13
                i32.const 6
                i32.shl
                i32.or
                local.get 12
                i32.or
                i32.const 1114112
                i32.eq
                br_if 3 (;@3;)
              end
              local.get 9
              local.get 4
              i32.sub
              local.get 8
              i32.add
              local.set 8
              local.get 7
              i32.const -1
              i32.add
              local.tee 7
              br_if 0 (;@5;)
            end
          end
          local.get 6
          local.get 9
          i32.eq
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 9
            i32.load8_s
            local.tee 4
            i32.const -1
            i32.gt_s
            br_if 0 (;@4;)
            block  ;; label = @5
              block  ;; label = @6
                local.get 9
                i32.const 1
                i32.add
                local.get 6
                i32.ne
                br_if 0 (;@6;)
                i32.const 0
                local.set 9
                local.get 6
                local.set 7
                br 1 (;@5;)
              end
              local.get 9
              i32.const 2
              i32.add
              local.set 7
              local.get 9
              i32.load8_u offset=1
              i32.const 63
              i32.and
              i32.const 6
              i32.shl
              local.set 9
            end
            local.get 4
            i32.const 255
            i32.and
            i32.const 224
            i32.lt_u
            br_if 0 (;@4;)
            block  ;; label = @5
              block  ;; label = @6
                local.get 7
                local.get 6
                i32.ne
                br_if 0 (;@6;)
                i32.const 0
                local.set 7
                local.get 6
                local.set 10
                br 1 (;@5;)
              end
              local.get 7
              i32.const 1
              i32.add
              local.set 10
              local.get 7
              i32.load8_u
              i32.const 63
              i32.and
              local.set 7
            end
            local.get 4
            i32.const 255
            i32.and
            i32.const 240
            i32.lt_u
            br_if 0 (;@4;)
            local.get 4
            i32.const 255
            i32.and
            local.set 4
            local.get 7
            local.get 9
            i32.or
            local.set 9
            block  ;; label = @5
              block  ;; label = @6
                local.get 10
                local.get 6
                i32.ne
                br_if 0 (;@6;)
                i32.const 0
                local.set 6
                br 1 (;@5;)
              end
              local.get 10
              i32.load8_u
              i32.const 63
              i32.and
              local.set 6
            end
            local.get 9
            i32.const 6
            i32.shl
            local.get 4
            i32.const 18
            i32.shl
            i32.const 1835008
            i32.and
            i32.or
            local.get 6
            i32.or
            i32.const 1114112
            i32.eq
            br_if 1 (;@3;)
          end
          block  ;; label = @4
            block  ;; label = @5
              local.get 8
              i32.eqz
              br_if 0 (;@5;)
              local.get 8
              local.get 2
              i32.eq
              br_if 0 (;@5;)
              i32.const 0
              local.set 4
              local.get 8
              local.get 2
              i32.ge_u
              br_if 1 (;@4;)
              local.get 1
              local.get 8
              i32.add
              i32.load8_s
              i32.const -64
              i32.lt_s
              br_if 1 (;@4;)
            end
            local.get 1
            local.set 4
          end
          local.get 8
          local.get 2
          local.get 4
          select
          local.set 2
          local.get 4
          local.get 1
          local.get 4
          select
          local.set 1
        end
        local.get 5
        br_if 0 (;@2;)
        local.get 0
        i32.load offset=24
        local.get 1
        local.get 2
        local.get 0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 0)
        local.set 4
        br 1 (;@1;)
      end
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.eqz
            br_if 0 (;@4;)
            i32.const 0
            local.set 4
            i32.const 0
            local.set 9
            loop  ;; label = @5
              local.get 9
              local.get 1
              local.get 4
              i32.add
              i32.load8_u
              i32.const 192
              i32.and
              i32.const 128
              i32.eq
              i32.add
              local.set 9
              local.get 2
              local.get 4
              i32.const 1
              i32.add
              local.tee 4
              i32.ne
              br_if 0 (;@5;)
            end
            local.get 2
            local.get 9
            i32.sub
            local.get 0
            i32.load offset=12
            local.tee 8
            i32.ge_u
            br_if 1 (;@3;)
            i32.const 0
            local.set 4
            i32.const 0
            local.set 9
            loop  ;; label = @5
              local.get 9
              local.get 1
              local.get 4
              i32.add
              i32.load8_u
              i32.const 192
              i32.and
              i32.const 128
              i32.eq
              i32.add
              local.set 9
              local.get 2
              local.get 4
              i32.const 1
              i32.add
              local.tee 4
              i32.ne
              br_if 0 (;@5;)
              br 3 (;@2;)
            end
          end
          i32.const 0
          local.set 9
          local.get 0
          i32.load offset=12
          local.tee 8
          br_if 1 (;@2;)
        end
        local.get 0
        i32.load offset=24
        local.get 1
        local.get 2
        local.get 0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 0)
        local.set 4
        br 1 (;@1;)
      end
      local.get 3
      i32.const 8
      i32.add
      local.get 0
      local.get 8
      local.get 2
      i32.sub
      local.get 9
      i32.add
      i32.const 0
      call $_ZN4core3fmt9Formatter7padding17h7a1896b194aaa2f4E
      i32.const 1
      local.set 4
      local.get 3
      i32.load offset=8
      local.tee 9
      i32.const 1114112
      i32.eq
      br_if 0 (;@1;)
      local.get 3
      i32.load offset=12
      local.set 8
      local.get 0
      i32.load offset=24
      local.get 1
      local.get 2
      local.get 0
      i32.const 28
      i32.add
      i32.load
      i32.load offset=12
      call_indirect (type 0)
      br_if 0 (;@1;)
      local.get 8
      i32.const 1
      i32.add
      local.set 4
      local.get 0
      i32.load offset=28
      local.set 2
      local.get 0
      i32.load offset=24
      local.set 1
      loop  ;; label = @2
        block  ;; label = @3
          local.get 4
          i32.const -1
          i32.add
          local.tee 4
          br_if 0 (;@3;)
          i32.const 0
          local.set 4
          br 2 (;@1;)
        end
        local.get 1
        local.get 9
        local.get 2
        i32.load offset=16
        call_indirect (type 1)
        i32.eqz
        br_if 0 (;@2;)
      end
      i32.const 1
      local.set 4
    end
    local.get 3
    i32.const 16
    i32.add
    global.set 0
    local.get 4)
  (func $_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17h72030837644d4392E (type 1) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i64)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    i32.const 1
    local.set 3
    block  ;; label = @1
      local.get 1
      i32.load offset=24
      i32.const 39
      local.get 1
      i32.const 28
      i32.add
      i32.load
      i32.load offset=16
      call_indirect (type 1)
      br_if 0 (;@1;)
      i32.const 2
      local.set 4
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 0
                  i32.load
                  local.tee 0
                  i32.const 33
                  i32.gt_s
                  br_if 0 (;@7;)
                  i32.const 116
                  local.set 5
                  local.get 0
                  i32.const -9
                  i32.add
                  br_table 5 (;@2;) 2 (;@5;) 3 (;@4;) 3 (;@4;) 1 (;@6;) 3 (;@4;)
                end
                block  ;; label = @7
                  local.get 0
                  i32.const 34
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 0
                  i32.const 39
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 0
                  i32.const 92
                  i32.ne
                  br_if 3 (;@4;)
                end
                br 3 (;@3;)
              end
              i32.const 114
              local.set 5
              br 3 (;@2;)
            end
            i32.const 110
            local.set 5
            br 2 (;@2;)
          end
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              call $_ZN4core7unicode12unicode_data15grapheme_extend6lookup17h9eaf4e302a998ac5E
              i32.eqz
              br_if 0 (;@5;)
              local.get 0
              i32.const 1
              i32.or
              i32.clz
              i32.const 2
              i32.shr_u
              i32.const 7
              i32.xor
              i64.extend_i32_u
              i64.const 21474836480
              i64.or
              local.set 6
              br 1 (;@4;)
            end
            block  ;; label = @5
              local.get 0
              call $_ZN4core7unicode9printable12is_printable17hdf644931c213b146E
              i32.eqz
              br_if 0 (;@5;)
              i32.const 1
              local.set 4
              br 2 (;@3;)
            end
            local.get 0
            i32.const 1
            i32.or
            i32.clz
            i32.const 2
            i32.shr_u
            i32.const 7
            i32.xor
            i64.extend_i32_u
            i64.const 21474836480
            i64.or
            local.set 6
          end
          i32.const 3
          local.set 4
        end
        local.get 0
        local.set 5
      end
      local.get 2
      i32.const 8
      i32.add
      local.get 6
      i64.store
      local.get 2
      local.get 5
      i32.store offset=4
      local.get 2
      local.get 4
      i32.store
      loop  ;; label = @2
        block  ;; label = @3
          local.get 2
          call $_ZN82_$LT$core..char..EscapeDebug$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he3ef662854a862cdE
          local.tee 0
          i32.const 1114112
          i32.ne
          br_if 0 (;@3;)
          local.get 1
          i32.load offset=24
          i32.const 39
          local.get 1
          i32.load offset=28
          i32.load offset=16
          call_indirect (type 1)
          local.set 3
          br 2 (;@1;)
        end
        local.get 1
        i32.load offset=24
        local.get 0
        local.get 1
        i32.load offset=28
        i32.load offset=16
        call_indirect (type 1)
        i32.eqz
        br_if 0 (;@2;)
      end
    end
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 3)
  (func $_ZN82_$LT$core..char..EscapeDebug$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he3ef662854a862cdE (type 4) (param i32) (result i32)
    (local i32 i32)
    i32.const 1114112
    local.set 1
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            i32.load
            br_table 3 (;@1;) 2 (;@2;) 1 (;@3;) 0 (;@4;) 3 (;@1;)
          end
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 0
                    i32.const 12
                    i32.add
                    i32.load8_u
                    br_table 7 (;@1;) 4 (;@4;) 3 (;@5;) 2 (;@6;) 1 (;@7;) 0 (;@8;) 7 (;@1;)
                  end
                  local.get 0
                  i32.const 4
                  i32.store8 offset=12
                  i32.const 92
                  return
                end
                local.get 0
                i32.const 3
                i32.store8 offset=12
                i32.const 117
                return
              end
              local.get 0
              i32.const 2
              i32.store8 offset=12
              i32.const 123
              return
            end
            i32.const 48
            i32.const 87
            local.get 0
            i32.load offset=4
            local.get 0
            i32.const 8
            i32.add
            i32.load
            local.tee 1
            i32.const 2
            i32.shl
            i32.const 28
            i32.and
            i32.shr_u
            i32.const 15
            i32.and
            local.tee 2
            i32.const 10
            i32.lt_u
            select
            local.get 2
            i32.add
            local.set 2
            block  ;; label = @5
              local.get 1
              i32.eqz
              br_if 0 (;@5;)
              local.get 0
              local.get 1
              i32.const -1
              i32.add
              i32.store offset=8
              local.get 2
              return
            end
            local.get 0
            i32.const 1
            i32.store8 offset=12
            local.get 2
            return
          end
          local.get 0
          i32.const 0
          i32.store8 offset=12
          i32.const 125
          return
        end
        local.get 0
        i32.const 1
        i32.store
        i32.const 92
        return
      end
      local.get 0
      i32.const 0
      i32.store
      local.get 0
      i32.load offset=4
      local.set 1
    end
    local.get 1)
  (func $_ZN53_$LT$core..fmt..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17hfb3d464953e3231aE (type 1) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 1
    i32.load offset=24
    i32.const 14068
    i32.const 5
    local.get 1
    i32.const 28
    i32.add
    i32.load
    i32.load offset=12
    call_indirect (type 0)
    i32.store8 offset=8
    local.get 2
    local.get 1
    i32.store
    local.get 2
    i32.const 0
    i32.store8 offset=9
    local.get 2
    i32.const 0
    i32.store offset=4
    local.get 2
    call $_ZN4core3fmt8builders10DebugTuple6finish17h45fab94fff212740E
    local.set 1
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 1)
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h997443bf2251431aE (type 1) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 128
    i32.sub
    local.tee 2
    global.set 0
    local.get 0
    i32.load
    local.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.load
              local.tee 3
              i32.const 16
              i32.and
              br_if 0 (;@5;)
              local.get 0
              i32.load
              local.set 4
              local.get 3
              i32.const 32
              i32.and
              br_if 1 (;@4;)
              local.get 4
              i64.extend_i32_u
              i32.const 1
              local.get 1
              call $_ZN4core3fmt3num3imp7fmt_u6417h53be2cf8685583ddE
              local.set 0
              br 2 (;@3;)
            end
            local.get 0
            i32.load
            local.set 4
            i32.const 0
            local.set 0
            loop  ;; label = @5
              local.get 2
              local.get 0
              i32.add
              i32.const 127
              i32.add
              local.get 4
              i32.const 15
              i32.and
              local.tee 3
              i32.const 48
              i32.or
              local.get 3
              i32.const 87
              i32.add
              local.get 3
              i32.const 10
              i32.lt_u
              select
              i32.store8
              local.get 0
              i32.const -1
              i32.add
              local.set 0
              local.get 4
              i32.const 4
              i32.shr_u
              local.tee 4
              br_if 0 (;@5;)
            end
            local.get 0
            i32.const 128
            i32.add
            local.tee 4
            i32.const 129
            i32.ge_u
            br_if 2 (;@2;)
            local.get 1
            i32.const 1
            i32.const 10976
            i32.const 2
            local.get 2
            local.get 0
            i32.add
            i32.const 128
            i32.add
            i32.const 0
            local.get 0
            i32.sub
            call $_ZN4core3fmt9Formatter12pad_integral17h07aef0d894bb255dE
            local.set 0
            br 1 (;@3;)
          end
          i32.const 0
          local.set 0
          loop  ;; label = @4
            local.get 2
            local.get 0
            i32.add
            i32.const 127
            i32.add
            local.get 4
            i32.const 15
            i32.and
            local.tee 3
            i32.const 48
            i32.or
            local.get 3
            i32.const 55
            i32.add
            local.get 3
            i32.const 10
            i32.lt_u
            select
            i32.store8
            local.get 0
            i32.const -1
            i32.add
            local.set 0
            local.get 4
            i32.const 4
            i32.shr_u
            local.tee 4
            br_if 0 (;@4;)
          end
          local.get 0
          i32.const 128
          i32.add
          local.tee 4
          i32.const 129
          i32.ge_u
          br_if 2 (;@1;)
          local.get 1
          i32.const 1
          i32.const 10976
          i32.const 2
          local.get 2
          local.get 0
          i32.add
          i32.const 128
          i32.add
          i32.const 0
          local.get 0
          i32.sub
          call $_ZN4core3fmt9Formatter12pad_integral17h07aef0d894bb255dE
          local.set 0
        end
        local.get 2
        i32.const 128
        i32.add
        global.set 0
        local.get 0
        return
      end
      local.get 4
      i32.const 128
      i32.const 10960
      call $_ZN4core5slice5index26slice_start_index_len_fail17h096dafa4b2741edeE
      unreachable
    end
    local.get 4
    i32.const 128
    i32.const 10960
    call $_ZN4core5slice5index26slice_start_index_len_fail17h096dafa4b2741edeE
    unreachable)
  (func $rust_begin_unwind (type 9) (param i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 0
    i32.load offset=12
    local.set 2
    local.get 0
    i32.load offset=8
    call $_ZN4core6option15Option$LT$T$GT$6unwrap17h3544e7a036042d11E
    local.set 3
    local.get 1
    local.get 2
    i32.store offset=8
    local.get 1
    local.get 0
    i32.store offset=4
    local.get 1
    local.get 3
    i32.store
    local.get 1
    call $_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17hb408146a39027d78E
    unreachable)
  (func $_ZN4core3ptr13drop_in_place17hc4ce7ec6fd1fe8d0E (type 9) (param i32))
  (func $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17hf545e9d77a58de68E (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 3
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.eqz
            br_if 0 (;@4;)
            local.get 3
            i32.const 40
            i32.add
            local.set 4
            loop  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.load offset=8
                i32.load8_u
                i32.eqz
                br_if 0 (;@6;)
                local.get 0
                i32.load
                i32.const 14158
                i32.const 4
                local.get 0
                i32.load offset=4
                i32.load offset=12
                call_indirect (type 0)
                br_if 4 (;@2;)
              end
              local.get 3
              i32.const 10
              i32.store offset=40
              local.get 3
              i64.const 4294967306
              i64.store offset=32
              local.get 3
              local.get 2
              i32.store offset=28
              i32.const 0
              local.set 5
              local.get 3
              i32.const 0
              i32.store offset=24
              local.get 3
              local.get 2
              i32.store offset=20
              local.get 3
              local.get 1
              i32.store offset=16
              i32.const 1
              local.set 6
              local.get 1
              local.set 7
              local.get 2
              local.set 8
              local.get 2
              local.set 9
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    loop  ;; label = @9
                      local.get 7
                      local.get 5
                      i32.add
                      local.set 7
                      local.get 6
                      local.get 3
                      i32.const 16
                      i32.add
                      i32.add
                      i32.const 23
                      i32.add
                      i32.load8_u
                      local.set 10
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 8
                              i32.const 7
                              i32.gt_u
                              br_if 0 (;@13;)
                              local.get 8
                              i32.eqz
                              br_if 1 (;@12;)
                              local.get 9
                              local.get 5
                              i32.sub
                              local.set 11
                              i32.const 0
                              local.set 8
                              loop  ;; label = @14
                                local.get 7
                                local.get 8
                                i32.add
                                i32.load8_u
                                local.get 10
                                i32.const 255
                                i32.and
                                i32.eq
                                br_if 4 (;@10;)
                                local.get 11
                                local.get 8
                                i32.const 1
                                i32.add
                                local.tee 8
                                i32.ne
                                br_if 0 (;@14;)
                                br 2 (;@12;)
                              end
                            end
                            local.get 3
                            i32.const 8
                            i32.add
                            local.get 10
                            local.get 7
                            local.get 8
                            call $_ZN4core5slice6memchr19memchr_general_case17h5889cd0f1abb24d8E
                            local.get 3
                            i32.load offset=8
                            i32.const 1
                            i32.eq
                            br_if 1 (;@11;)
                            local.get 3
                            i32.load offset=28
                            local.set 9
                          end
                          local.get 3
                          local.get 9
                          i32.store offset=24
                          br 4 (;@7;)
                        end
                        local.get 3
                        i32.load offset=12
                        local.set 8
                        local.get 3
                        i32.load offset=36
                        local.set 6
                        local.get 3
                        i32.load offset=24
                        local.set 5
                      end
                      local.get 3
                      local.get 5
                      local.get 8
                      i32.add
                      i32.const 1
                      i32.add
                      local.tee 5
                      i32.store offset=24
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 5
                            local.get 6
                            i32.ge_u
                            br_if 0 (;@12;)
                            local.get 3
                            i32.load offset=20
                            local.set 8
                            br 1 (;@11;)
                          end
                          local.get 3
                          i32.load offset=20
                          local.tee 8
                          local.get 5
                          i32.lt_u
                          br_if 0 (;@11;)
                          local.get 6
                          i32.const 5
                          i32.ge_u
                          br_if 3 (;@8;)
                          local.get 3
                          i32.load offset=16
                          local.get 5
                          local.get 6
                          i32.sub
                          local.tee 10
                          i32.add
                          local.tee 7
                          local.get 4
                          i32.eq
                          br_if 1 (;@10;)
                          local.get 7
                          local.get 4
                          local.get 6
                          call $memcmp
                          i32.eqz
                          br_if 1 (;@10;)
                        end
                        local.get 3
                        i32.load offset=28
                        local.tee 9
                        local.get 5
                        i32.lt_u
                        br_if 3 (;@7;)
                        local.get 8
                        local.get 9
                        i32.lt_u
                        br_if 3 (;@7;)
                        local.get 9
                        local.get 5
                        i32.sub
                        local.set 8
                        local.get 3
                        i32.load offset=16
                        local.set 7
                        br 1 (;@9;)
                      end
                    end
                    local.get 0
                    i32.load offset=8
                    i32.const 1
                    i32.store8
                    local.get 10
                    i32.const 1
                    i32.add
                    local.set 8
                    br 2 (;@6;)
                  end
                  local.get 6
                  i32.const 4
                  i32.const 14164
                  call $_ZN4core5slice5index24slice_end_index_len_fail17h39f0c0387bdaee32E
                  unreachable
                end
                local.get 0
                i32.load offset=8
                i32.const 0
                i32.store8
                local.get 2
                local.set 8
              end
              local.get 0
              i32.load offset=4
              local.set 7
              local.get 0
              i32.load
              local.set 10
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 8
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 2
                    local.get 8
                    i32.eq
                    br_if 0 (;@8;)
                    block  ;; label = @9
                      local.get 2
                      local.get 8
                      i32.le_u
                      br_if 0 (;@9;)
                      local.get 1
                      local.get 8
                      i32.add
                      local.tee 11
                      i32.load8_s
                      i32.const -65
                      i32.gt_s
                      br_if 2 (;@7;)
                    end
                    local.get 1
                    local.get 2
                    i32.const 0
                    local.get 8
                    i32.const 14180
                    call $_ZN4core3str16slice_error_fail17hae7eb9a1c93dfe0dE
                    unreachable
                  end
                  local.get 10
                  local.get 1
                  local.get 8
                  local.get 7
                  i32.load offset=12
                  call_indirect (type 0)
                  br_if 5 (;@2;)
                  br 1 (;@6;)
                end
                local.get 10
                local.get 1
                local.get 8
                local.get 7
                i32.load offset=12
                call_indirect (type 0)
                br_if 4 (;@2;)
                local.get 11
                i32.load8_s
                i32.const -65
                i32.le_s
                br_if 3 (;@3;)
              end
              local.get 1
              local.get 8
              i32.add
              local.set 1
              local.get 2
              local.get 8
              i32.sub
              local.tee 2
              br_if 0 (;@5;)
            end
          end
          i32.const 0
          local.set 8
          br 2 (;@1;)
        end
        local.get 1
        local.get 2
        local.get 8
        local.get 2
        i32.const 14196
        call $_ZN4core3str16slice_error_fail17hae7eb9a1c93dfe0dE
        unreachable
      end
      i32.const 1
      local.set 8
    end
    local.get 3
    i32.const 48
    i32.add
    global.set 0
    local.get 8)
  (func $_ZN4core3str16slice_error_fail17hae7eb9a1c93dfe0dE (type 7) (param i32 i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 112
    i32.sub
    local.tee 5
    global.set 0
    local.get 5
    local.get 3
    i32.store offset=12
    local.get 5
    local.get 2
    i32.store offset=8
    i32.const 1
    local.set 6
    local.get 1
    local.set 7
    block  ;; label = @1
      local.get 1
      i32.const 257
      i32.lt_u
      br_if 0 (;@1;)
      i32.const 0
      local.get 1
      i32.sub
      local.set 8
      i32.const 256
      local.set 9
      loop  ;; label = @2
        block  ;; label = @3
          local.get 9
          local.get 1
          i32.ge_u
          br_if 0 (;@3;)
          i32.const 0
          local.set 6
          local.get 0
          local.get 9
          i32.add
          i32.load8_s
          i32.const -65
          i32.le_s
          br_if 0 (;@3;)
          local.get 9
          local.set 7
          br 2 (;@1;)
        end
        local.get 9
        i32.const -1
        i32.add
        local.set 7
        i32.const 0
        local.set 6
        local.get 9
        i32.const 1
        i32.eq
        br_if 1 (;@1;)
        local.get 8
        local.get 9
        i32.add
        local.set 10
        local.get 7
        local.set 9
        local.get 10
        i32.const 1
        i32.ne
        br_if 0 (;@2;)
      end
    end
    local.get 5
    local.get 7
    i32.store offset=20
    local.get 5
    local.get 0
    i32.store offset=16
    local.get 5
    i32.const 0
    i32.const 5
    local.get 6
    select
    i32.store offset=28
    local.get 5
    i32.const 15240
    i32.const 14480
    local.get 6
    select
    i32.store offset=24
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        local.get 1
        i32.gt_u
        br_if 0 (;@2;)
        local.get 3
        local.get 1
        i32.le_u
        br_if 1 (;@1;)
        local.get 3
        local.set 2
      end
      local.get 5
      local.get 2
      i32.store offset=40
      local.get 5
      i32.const 48
      i32.add
      i32.const 20
      i32.add
      i32.const 3
      i32.store
      local.get 5
      i32.const 72
      i32.add
      i32.const 20
      i32.add
      i32.const 4
      i32.store
      local.get 5
      i32.const 84
      i32.add
      i32.const 4
      i32.store
      local.get 5
      i64.const 3
      i64.store offset=52 align=4
      local.get 5
      i32.const 14488
      i32.store offset=48
      local.get 5
      i32.const 6
      i32.store offset=76
      local.get 5
      local.get 5
      i32.const 72
      i32.add
      i32.store offset=64
      local.get 5
      local.get 5
      i32.const 24
      i32.add
      i32.store offset=88
      local.get 5
      local.get 5
      i32.const 16
      i32.add
      i32.store offset=80
      local.get 5
      local.get 5
      i32.const 40
      i32.add
      i32.store offset=72
      local.get 5
      i32.const 48
      i32.add
      local.get 4
      call $_ZN4core9panicking9panic_fmt17hdf8d0243e0d4fcebE
      unreachable
    end
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          local.get 3
          i32.gt_u
          br_if 0 (;@3;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              i32.eqz
              br_if 0 (;@5;)
              local.get 2
              local.get 1
              i32.eq
              br_if 0 (;@5;)
              local.get 2
              local.get 1
              i32.ge_u
              br_if 1 (;@4;)
              local.get 0
              local.get 2
              i32.add
              i32.load8_s
              i32.const -64
              i32.lt_s
              br_if 1 (;@4;)
            end
            local.get 3
            local.set 2
          end
          local.get 5
          local.get 2
          i32.store offset=32
          local.get 2
          i32.eqz
          br_if 1 (;@2;)
          local.get 2
          local.get 1
          i32.eq
          br_if 1 (;@2;)
          local.get 1
          i32.const 1
          i32.add
          local.set 9
          loop  ;; label = @4
            block  ;; label = @5
              local.get 2
              local.get 1
              i32.ge_u
              br_if 0 (;@5;)
              local.get 0
              local.get 2
              i32.add
              i32.load8_s
              i32.const -64
              i32.ge_s
              br_if 3 (;@2;)
            end
            local.get 2
            i32.const -1
            i32.add
            local.set 6
            local.get 2
            i32.const 1
            i32.eq
            br_if 3 (;@1;)
            local.get 9
            local.get 2
            i32.eq
            local.set 7
            local.get 6
            local.set 2
            local.get 7
            i32.eqz
            br_if 0 (;@4;)
            br 3 (;@1;)
          end
        end
        local.get 5
        i32.const 100
        i32.add
        i32.const 4
        i32.store
        local.get 5
        i32.const 72
        i32.add
        i32.const 20
        i32.add
        i32.const 4
        i32.store
        local.get 5
        i32.const 84
        i32.add
        i32.const 6
        i32.store
        local.get 5
        i32.const 48
        i32.add
        i32.const 20
        i32.add
        i32.const 4
        i32.store
        local.get 5
        i64.const 4
        i64.store offset=52 align=4
        local.get 5
        i32.const 14512
        i32.store offset=48
        local.get 5
        i32.const 6
        i32.store offset=76
        local.get 5
        local.get 5
        i32.const 72
        i32.add
        i32.store offset=64
        local.get 5
        local.get 5
        i32.const 24
        i32.add
        i32.store offset=96
        local.get 5
        local.get 5
        i32.const 16
        i32.add
        i32.store offset=88
        local.get 5
        local.get 5
        i32.const 12
        i32.add
        i32.store offset=80
        local.get 5
        local.get 5
        i32.const 8
        i32.add
        i32.store offset=72
        local.get 5
        i32.const 48
        i32.add
        local.get 4
        call $_ZN4core9panicking9panic_fmt17hdf8d0243e0d4fcebE
        unreachable
      end
      local.get 2
      local.set 6
    end
    block  ;; label = @1
      local.get 6
      local.get 1
      i32.eq
      br_if 0 (;@1;)
      i32.const 1
      local.set 9
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              local.get 6
              i32.add
              local.tee 7
              i32.load8_s
              local.tee 2
              i32.const -1
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              local.set 9
              local.get 0
              local.get 1
              i32.add
              local.tee 1
              local.set 3
              block  ;; label = @6
                local.get 7
                i32.const 1
                i32.add
                local.get 1
                i32.eq
                br_if 0 (;@6;)
                local.get 7
                i32.const 2
                i32.add
                local.set 3
                local.get 7
                i32.load8_u offset=1
                i32.const 63
                i32.and
                local.set 9
              end
              local.get 2
              i32.const 31
              i32.and
              local.set 7
              local.get 2
              i32.const 255
              i32.and
              i32.const 223
              i32.gt_u
              br_if 1 (;@4;)
              local.get 9
              local.get 7
              i32.const 6
              i32.shl
              i32.or
              local.set 1
              br 2 (;@3;)
            end
            local.get 5
            local.get 2
            i32.const 255
            i32.and
            i32.store offset=36
            br 2 (;@2;)
          end
          i32.const 0
          local.set 0
          local.get 1
          local.set 8
          block  ;; label = @4
            local.get 3
            local.get 1
            i32.eq
            br_if 0 (;@4;)
            local.get 3
            i32.const 1
            i32.add
            local.set 8
            local.get 3
            i32.load8_u
            i32.const 63
            i32.and
            local.set 0
          end
          local.get 0
          local.get 9
          i32.const 6
          i32.shl
          i32.or
          local.set 9
          block  ;; label = @4
            local.get 2
            i32.const 255
            i32.and
            i32.const 240
            i32.ge_u
            br_if 0 (;@4;)
            local.get 9
            local.get 7
            i32.const 12
            i32.shl
            i32.or
            local.set 1
            br 1 (;@3;)
          end
          i32.const 0
          local.set 2
          block  ;; label = @4
            local.get 8
            local.get 1
            i32.eq
            br_if 0 (;@4;)
            local.get 8
            i32.load8_u
            i32.const 63
            i32.and
            local.set 2
          end
          local.get 9
          i32.const 6
          i32.shl
          local.get 7
          i32.const 18
          i32.shl
          i32.const 1835008
          i32.and
          i32.or
          local.get 2
          i32.or
          local.tee 1
          i32.const 1114112
          i32.eq
          br_if 2 (;@1;)
        end
        local.get 5
        local.get 1
        i32.store offset=36
        i32.const 1
        local.set 9
        local.get 1
        i32.const 128
        i32.lt_u
        br_if 0 (;@2;)
        i32.const 2
        local.set 9
        local.get 1
        i32.const 2048
        i32.lt_u
        br_if 0 (;@2;)
        i32.const 3
        i32.const 4
        local.get 1
        i32.const 65536
        i32.lt_u
        select
        local.set 9
      end
      local.get 5
      local.get 6
      i32.store offset=40
      local.get 5
      local.get 9
      local.get 6
      i32.add
      i32.store offset=44
      local.get 5
      i32.const 48
      i32.add
      i32.const 20
      i32.add
      i32.const 5
      i32.store
      local.get 5
      i32.const 108
      i32.add
      i32.const 4
      i32.store
      local.get 5
      i32.const 100
      i32.add
      i32.const 4
      i32.store
      local.get 5
      i32.const 72
      i32.add
      i32.const 20
      i32.add
      i32.const 8
      i32.store
      local.get 5
      i32.const 84
      i32.add
      i32.const 9
      i32.store
      local.get 5
      i64.const 5
      i64.store offset=52 align=4
      local.get 5
      i32.const 14544
      i32.store offset=48
      local.get 5
      i32.const 6
      i32.store offset=76
      local.get 5
      local.get 5
      i32.const 72
      i32.add
      i32.store offset=64
      local.get 5
      local.get 5
      i32.const 24
      i32.add
      i32.store offset=104
      local.get 5
      local.get 5
      i32.const 16
      i32.add
      i32.store offset=96
      local.get 5
      local.get 5
      i32.const 40
      i32.add
      i32.store offset=88
      local.get 5
      local.get 5
      i32.const 36
      i32.add
      i32.store offset=80
      local.get 5
      local.get 5
      i32.const 32
      i32.add
      i32.store offset=72
      local.get 5
      i32.const 48
      i32.add
      local.get 4
      call $_ZN4core9panicking9panic_fmt17hdf8d0243e0d4fcebE
      unreachable
    end
    i32.const 15197
    i32.const 43
    local.get 4
    call $_ZN4core9panicking5panic17h1f31dfdebac1729fE
    unreachable)
  (func $_ZN4core3fmt8builders10DebugTuple5field17h63d9c6bcee733729E (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i64 i64)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 3
    global.set 0
    i32.const 1
    local.set 4
    block  ;; label = @1
      local.get 0
      i32.load8_u offset=8
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.set 5
      block  ;; label = @2
        local.get 0
        i32.load
        local.tee 6
        i32.load8_u
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        i32.const 1
        local.set 4
        local.get 6
        i32.load offset=24
        i32.const 14443
        i32.const 14449
        local.get 5
        select
        i32.const 2
        i32.const 1
        local.get 5
        select
        local.get 6
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 0)
        br_if 1 (;@1;)
        local.get 1
        local.get 0
        i32.load
        local.get 2
        i32.load offset=12
        call_indirect (type 1)
        local.set 4
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 5
        br_if 0 (;@2;)
        i32.const 1
        local.set 4
        local.get 6
        i32.load offset=24
        i32.const 14450
        i32.const 2
        local.get 6
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 0)
        br_if 1 (;@1;)
        local.get 0
        i32.load
        local.set 6
      end
      i32.const 1
      local.set 4
      local.get 3
      i32.const 1
      i32.store8 offset=23
      local.get 3
      i32.const 52
      i32.add
      i32.const 14044
      i32.store
      local.get 3
      local.get 6
      i64.load offset=24 align=4
      i64.store offset=8
      local.get 3
      local.get 3
      i32.const 23
      i32.add
      i32.store offset=16
      local.get 6
      i64.load offset=8 align=4
      local.set 7
      local.get 6
      i64.load offset=16 align=4
      local.set 8
      local.get 3
      local.get 6
      i32.load8_u offset=32
      i32.store8 offset=56
      local.get 3
      local.get 8
      i64.store offset=40
      local.get 3
      local.get 7
      i64.store offset=32
      local.get 3
      local.get 6
      i64.load align=4
      i64.store offset=24
      local.get 3
      local.get 3
      i32.const 8
      i32.add
      i32.store offset=48
      local.get 1
      local.get 3
      i32.const 24
      i32.add
      local.get 2
      i32.load offset=12
      call_indirect (type 1)
      br_if 0 (;@1;)
      local.get 3
      i32.load offset=48
      i32.const 14447
      i32.const 2
      local.get 3
      i32.load offset=52
      i32.load offset=12
      call_indirect (type 0)
      local.set 4
    end
    local.get 0
    local.get 4
    i32.store8 offset=8
    local.get 0
    local.get 0
    i32.load offset=4
    i32.const 1
    i32.add
    i32.store offset=4
    local.get 3
    i32.const 64
    i32.add
    global.set 0
    local.get 0)
  (func $_ZN4core3fmt8builders10DebugInner5entry17hc22595ec27448a67E (type 8) (param i32 i32 i32)
    (local i32 i32 i32 i64 i64)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 3
    global.set 0
    i32.const 1
    local.set 4
    block  ;; label = @1
      local.get 0
      i32.load8_u offset=4
      br_if 0 (;@1;)
      local.get 0
      i32.load8_u offset=5
      local.set 4
      block  ;; label = @2
        local.get 0
        i32.load
        local.tee 5
        i32.load8_u
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 4
          i32.const 255
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          i32.const 1
          local.set 4
          local.get 5
          i32.load offset=24
          i32.const 14443
          i32.const 2
          local.get 5
          i32.const 28
          i32.add
          i32.load
          i32.load offset=12
          call_indirect (type 0)
          br_if 2 (;@1;)
          local.get 0
          i32.load
          local.set 5
        end
        local.get 1
        local.get 5
        local.get 2
        i32.load offset=12
        call_indirect (type 1)
        local.set 4
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 4
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 1
        local.set 4
        local.get 5
        i32.load offset=24
        i32.const 15176
        i32.const 1
        local.get 5
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 0)
        br_if 1 (;@1;)
        local.get 0
        i32.load
        local.set 5
      end
      i32.const 1
      local.set 4
      local.get 3
      i32.const 1
      i32.store8 offset=23
      local.get 3
      i32.const 52
      i32.add
      i32.const 14044
      i32.store
      local.get 3
      local.get 5
      i64.load offset=24 align=4
      i64.store offset=8
      local.get 3
      local.get 3
      i32.const 23
      i32.add
      i32.store offset=16
      local.get 5
      i64.load offset=8 align=4
      local.set 6
      local.get 5
      i64.load offset=16 align=4
      local.set 7
      local.get 3
      local.get 5
      i32.load8_u offset=32
      i32.store8 offset=56
      local.get 3
      local.get 7
      i64.store offset=40
      local.get 3
      local.get 6
      i64.store offset=32
      local.get 3
      local.get 5
      i64.load align=4
      i64.store offset=24
      local.get 3
      local.get 3
      i32.const 8
      i32.add
      i32.store offset=48
      local.get 1
      local.get 3
      i32.const 24
      i32.add
      local.get 2
      i32.load offset=12
      call_indirect (type 1)
      br_if 0 (;@1;)
      local.get 3
      i32.load offset=48
      i32.const 14447
      i32.const 2
      local.get 3
      i32.load offset=52
      i32.load offset=12
      call_indirect (type 0)
      local.set 4
    end
    local.get 0
    i32.const 1
    i32.store8 offset=5
    local.get 0
    local.get 4
    i32.store8 offset=4
    local.get 3
    i32.const 64
    i32.add
    global.set 0)
  (func $_ZN4core3fmt5Write10write_char17h9885dd2687cf2480E (type 1) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 0
    i32.store offset=12
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 128
            i32.lt_u
            br_if 0 (;@4;)
            local.get 1
            i32.const 2048
            i32.lt_u
            br_if 1 (;@3;)
            local.get 1
            i32.const 65536
            i32.ge_u
            br_if 2 (;@2;)
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 224
            i32.or
            i32.store8 offset=12
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            i32.const 3
            local.set 1
            br 3 (;@1;)
          end
          local.get 2
          local.get 1
          i32.store8 offset=12
          i32.const 1
          local.set 1
          br 2 (;@1;)
        end
        local.get 2
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 192
        i32.or
        i32.store8 offset=12
        i32.const 2
        local.set 1
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=15
      local.get 2
      local.get 1
      i32.const 18
      i32.shr_u
      i32.const 240
      i32.or
      i32.store8 offset=12
      local.get 2
      local.get 1
      i32.const 6
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=14
      local.get 2
      local.get 1
      i32.const 12
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=13
      i32.const 4
      local.set 1
    end
    local.get 0
    local.get 2
    i32.const 12
    i32.add
    local.get 1
    call $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17hf545e9d77a58de68E
    local.set 1
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 1)
  (func $_ZN4core3fmt5Write9write_fmt17h3a716cb550662ed4E (type 1) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 0
    i32.store offset=4
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    i32.const 4
    i32.add
    i32.const 14456
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17hbe0dd8e9c0d9aeb7E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set 0
    local.get 1)
  (func $_ZN4core3ptr13drop_in_place17h48010a17ef1ba2d2E (type 9) (param i32))
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17hd10da3a7c84c86a1E (type 0) (param i32 i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    local.get 2
    call $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17hf545e9d77a58de68E)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17ha2b0ac3db39c866aE (type 1) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    call $_ZN4core3fmt5Write10write_char17h9885dd2687cf2480E)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17hd92bcf7983e2e3ccE (type 1) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 0
    i32.load
    i32.store offset=4
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    i32.const 4
    i32.add
    i32.const 14456
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17hbe0dd8e9c0d9aeb7E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set 0
    local.get 1)
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hde54dc4b6c8fa177E (type 1) (param i32 i32) (result i32)
    (local i32 i32 i32 i64)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 0
    i32.load
    local.tee 0
    i32.load offset=8
    local.set 3
    local.get 0
    i32.load
    local.set 0
    i32.const 1
    local.set 4
    local.get 2
    i64.const 4294967296
    i64.const 0
    local.get 1
    i32.load offset=24
    i32.const 14453
    i32.const 1
    local.get 1
    i32.const 28
    i32.add
    i32.load
    i32.load offset=12
    call_indirect (type 0)
    select
    local.tee 5
    local.get 1
    i64.extend_i32_u
    i64.or
    i64.store
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        br_if 0 (;@2;)
        local.get 5
        i64.const 32
        i64.shr_u
        i32.wrap_i64
        local.set 0
        br 1 (;@1;)
      end
      local.get 3
      i32.const 2
      i32.shl
      local.set 1
      loop  ;; label = @2
        local.get 2
        local.get 0
        i32.store offset=12
        local.get 2
        local.get 2
        i32.const 12
        i32.add
        i32.const 15028
        call $_ZN4core3fmt8builders10DebugInner5entry17hc22595ec27448a67E
        local.get 0
        i32.const 4
        i32.add
        local.set 0
        local.get 1
        i32.const -4
        i32.add
        local.tee 1
        br_if 0 (;@2;)
      end
      local.get 2
      i32.load8_u offset=4
      local.set 0
    end
    block  ;; label = @1
      local.get 0
      i32.const 255
      i32.and
      br_if 0 (;@1;)
      local.get 2
      i32.load
      local.tee 0
      i32.load offset=24
      i32.const 14454
      i32.const 1
      local.get 0
      i32.const 28
      i32.add
      i32.load
      i32.load offset=12
      call_indirect (type 0)
      local.set 4
    end
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 4)
  (func $_ZN5alloc3vec12Vec$LT$T$GT$17extend_from_slice17h032ebe9cd1c3b0dfE (type 8) (param i32 i32 i32)
    local.get 0
    local.get 0
    i32.load offset=8
    local.get 2
    call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve17h05b20ec8d167cd79E
    local.get 0
    i32.load
    local.get 0
    i32.load offset=8
    i32.const 2
    i32.shl
    i32.add
    local.get 1
    local.get 2
    i32.const 2
    i32.shl
    call $memcpy
    drop
    local.get 0
    local.get 0
    i32.load offset=8
    local.get 2
    i32.add
    i32.store offset=8)
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve17h05b20ec8d167cd79E (type 8) (param i32 i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.load offset=4
          local.tee 4
          local.get 1
          i32.sub
          local.get 2
          i32.ge_u
          br_if 0 (;@3;)
          local.get 1
          local.get 2
          i32.add
          local.tee 2
          local.get 1
          i32.lt_u
          br_if 2 (;@1;)
          local.get 4
          i32.const 1
          i32.shl
          local.tee 1
          local.get 2
          local.get 1
          local.get 2
          i32.gt_u
          select
          local.tee 1
          i32.const 4
          local.get 1
          i32.const 4
          i32.gt_u
          select
          local.tee 1
          i32.const 1073741823
          i32.and
          local.get 1
          i32.eq
          i32.const 2
          i32.shl
          local.set 2
          local.get 1
          i32.const 2
          i32.shl
          local.set 1
          block  ;; label = @4
            block  ;; label = @5
              local.get 4
              i32.eqz
              br_if 0 (;@5;)
              local.get 3
              i32.const 24
              i32.add
              i32.const 4
              i32.store
              local.get 3
              local.get 4
              i32.const 2
              i32.shl
              i32.store offset=20
              local.get 3
              local.get 0
              i32.load
              i32.store offset=16
              br 1 (;@4;)
            end
            local.get 3
            i32.const 0
            i32.store offset=16
          end
          local.get 3
          local.get 1
          local.get 2
          local.get 3
          i32.const 16
          i32.add
          call $_ZN5alloc7raw_vec11finish_grow17h3e91ad6198eee436E
          local.get 3
          i32.const 8
          i32.add
          i32.load
          local.set 1
          local.get 3
          i32.load offset=4
          local.set 2
          local.get 3
          i32.load
          i32.const 1
          i32.eq
          br_if 1 (;@2;)
          local.get 0
          local.get 2
          i32.store
          local.get 0
          local.get 1
          i32.const 2
          i32.shr_u
          i32.store offset=4
        end
        local.get 3
        i32.const 32
        i32.add
        global.set 0
        return
      end
      local.get 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      local.get 1
      call $_ZN5alloc5alloc18handle_alloc_error17h8e32739f1fa5c58cE
      unreachable
    end
    call $_ZN5alloc7raw_vec17capacity_overflow17hb5aace515ee70317E
    unreachable)
  (func $_ZN5alloc3vec12Vec$LT$T$GT$5drain17h8be220de23425f59E (type 5) (param i32 i32 i32 i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        local.get 3
        i32.gt_u
        br_if 0 (;@2;)
        local.get 1
        i32.load offset=8
        local.tee 4
        local.get 3
        i32.lt_u
        br_if 1 (;@1;)
        local.get 1
        local.get 2
        i32.store offset=8
        local.get 0
        local.get 1
        i32.store offset=16
        local.get 0
        local.get 3
        i32.store
        local.get 0
        local.get 4
        local.get 3
        i32.sub
        i32.store offset=4
        local.get 0
        i32.const 12
        i32.add
        local.get 1
        i32.load
        local.tee 1
        local.get 3
        i32.const 2
        i32.shl
        i32.add
        i32.store
        local.get 0
        local.get 1
        local.get 2
        i32.const 2
        i32.shl
        i32.add
        i32.store offset=8
        return
      end
      local.get 2
      local.get 3
      i32.const 14704
      call $_ZN4core5slice5index22slice_index_order_fail17h3b506dc87657b590E
      unreachable
    end
    local.get 3
    local.get 4
    i32.const 14704
    call $_ZN4core5slice5index24slice_end_index_len_fail17h39f0c0387bdaee32E
    unreachable)
  (func $_ZN5alloc3vec14Drain$LT$T$GT$4fill17h8a131803b354a583E (type 1) (param i32 i32) (result i32)
    (local i32 i32 i32)
    i32.const 1
    local.set 2
    block  ;; label = @1
      local.get 0
      i32.load offset=16
      local.tee 3
      i32.load offset=8
      local.tee 4
      local.get 0
      i32.load
      local.tee 0
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      i32.const 2
      i32.shl
      local.get 4
      i32.const 2
      i32.shl
      local.tee 0
      i32.sub
      local.set 4
      local.get 3
      i32.load
      local.get 0
      i32.add
      local.set 0
      loop  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.load
          local.tee 2
          local.get 1
          i32.load offset=4
          i32.ne
          br_if 0 (;@3;)
          i32.const 0
          return
        end
        local.get 1
        local.get 2
        i32.const 4
        i32.add
        i32.store
        local.get 0
        local.get 2
        i32.load
        i32.store
        i32.const 1
        local.set 2
        local.get 3
        local.get 3
        i32.load offset=8
        i32.const 1
        i32.add
        i32.store offset=8
        local.get 0
        i32.const 4
        i32.add
        local.set 0
        local.get 4
        i32.const -4
        i32.add
        local.tee 4
        br_if 0 (;@2;)
      end
    end
    local.get 2)
  (func $_ZN68_$LT$alloc..vec..Drain$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hbb7c9400224cfddaE (type 9) (param i32)
    (local i32 i32 i32 i32 i32)
    block  ;; label = @1
      local.get 0
      i32.load offset=8
      local.tee 1
      local.get 0
      i32.const 12
      i32.add
      i32.load
      local.tee 2
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      local.get 2
      local.get 1
      i32.sub
      i32.const -4
      i32.add
      i32.const -4
      i32.and
      local.get 1
      i32.add
      i32.const 4
      i32.add
      local.tee 1
      i32.store offset=8
      local.get 1
      local.get 2
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      local.get 2
      local.get 1
      i32.sub
      i32.const -4
      i32.add
      i32.const -4
      i32.and
      local.get 1
      i32.add
      i32.const 4
      i32.add
      i32.store offset=8
    end
    block  ;; label = @1
      local.get 0
      i32.load offset=4
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 0
        i32.load
        local.tee 3
        local.get 0
        i32.load offset=16
        local.tee 1
        i32.load offset=8
        local.tee 4
        i32.eq
        br_if 0 (;@2;)
        local.get 1
        i32.load
        local.tee 5
        local.get 4
        i32.const 2
        i32.shl
        i32.add
        local.get 5
        local.get 3
        i32.const 2
        i32.shl
        i32.add
        local.get 2
        i32.const 2
        i32.shl
        call $memmove
        drop
        local.get 0
        i32.load offset=4
        local.set 2
      end
      local.get 1
      local.get 2
      local.get 4
      i32.add
      i32.store offset=8
    end)
  (func $_ZN80_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..SpecExtend$LT$T$C$I$GT$$GT$11spec_extend17hcbe54f7519367931E (type 2) (param i32 i32)
    (local i32 i32 i32 i32)
    local.get 0
    local.get 0
    i32.load offset=8
    local.get 1
    i32.load offset=4
    local.get 1
    i32.load
    i32.sub
    i32.const 2
    i32.shr_u
    call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve17h05b20ec8d167cd79E
    local.get 0
    i32.load offset=8
    local.set 2
    block  ;; label = @1
      local.get 1
      i32.load
      local.tee 3
      local.get 1
      i32.load offset=4
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      i32.load
      local.set 4
      local.get 1
      local.get 3
      i32.const 4
      i32.add
      i32.store
      local.get 2
      i32.const 1
      i32.add
      local.set 5
      local.get 4
      local.get 2
      i32.const 2
      i32.shl
      i32.add
      local.set 2
      local.get 3
      i32.load
      local.set 3
      loop  ;; label = @2
        local.get 2
        local.get 3
        i32.store
        block  ;; label = @3
          local.get 1
          i32.load
          local.tee 3
          local.get 1
          i32.load offset=4
          i32.ne
          br_if 0 (;@3;)
          local.get 5
          local.set 2
          br 2 (;@1;)
        end
        local.get 1
        local.get 3
        i32.const 4
        i32.add
        i32.store
        local.get 5
        i32.const 1
        i32.add
        local.set 5
        local.get 2
        i32.const 4
        i32.add
        local.set 2
        local.get 3
        i32.load
        local.set 3
        br 0 (;@2;)
      end
    end
    local.get 0
    local.get 2
    i32.store offset=8)
  (func $_ZN4core3ptr13drop_in_place17hac568a8f4dd0fc8dE (type 9) (param i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.load
      br_if 0 (;@1;)
      local.get 0
      i32.const 8
      i32.add
      i32.load
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const 2
      i32.shl
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.get 1
      call $__rust_dealloc
    end)
  (func $_ZN64_$LT$icu_uniset..UnicodeSetError$u20$as$u20$core..fmt..Debug$GT$3fmt17hf5d167b28bb4dc42E (type 1) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load
        i32.const 1
        i32.ne
        br_if 0 (;@2;)
        local.get 2
        local.get 1
        i32.load offset=24
        i32.const 14971
        i32.const 12
        local.get 1
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 0)
        i32.store8 offset=8
        local.get 2
        local.get 1
        i32.store
        local.get 2
        i32.const 0
        i32.store8 offset=9
        local.get 2
        i32.const 0
        i32.store offset=4
        local.get 2
        local.get 0
        i32.const 4
        i32.add
        i32.store offset=12
        local.get 2
        local.get 2
        i32.const 12
        i32.add
        i32.const 14984
        call $_ZN4core3fmt8builders10DebugTuple5field17h63d9c6bcee733729E
        local.set 1
        local.get 2
        local.get 0
        i32.const 8
        i32.add
        i32.store offset=12
        local.get 1
        local.get 2
        i32.const 12
        i32.add
        i32.const 14984
        call $_ZN4core3fmt8builders10DebugTuple5field17h63d9c6bcee733729E
        call $_ZN4core3fmt8builders10DebugTuple6finish17h45fab94fff212740E
        local.set 1
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.load offset=24
      i32.const 15000
      i32.const 10
      local.get 1
      i32.const 28
      i32.add
      i32.load
      i32.load offset=12
      call_indirect (type 0)
      i32.store8 offset=8
      local.get 2
      local.get 1
      i32.store
      local.get 2
      i32.const 0
      i32.store8 offset=9
      local.get 2
      i32.const 0
      i32.store offset=4
      local.get 2
      local.get 0
      i32.const 4
      i32.add
      i32.store offset=12
      local.get 2
      local.get 2
      i32.const 12
      i32.add
      i32.const 15012
      call $_ZN4core3fmt8builders10DebugTuple5field17h63d9c6bcee733729E
      call $_ZN4core3fmt8builders10DebugTuple6finish17h45fab94fff212740E
      local.set 1
    end
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 1)
  (func $_ZN4core3ptr13drop_in_place17h18f90f79f1b1f11eE.628 (type 9) (param i32))
  (func $_ZN4core3ptr13drop_in_place17h74320d6e375583feE (type 9) (param i32))
  (func $_ZN4core3ptr13drop_in_place17h74320d6e375583feE.651 (type 9) (param i32))
  (func $_ZN4core6option15Option$LT$T$GT$6unwrap17h3544e7a036042d11E (type 4) (param i32) (result i32)
    block  ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      i32.const 15197
      i32.const 43
      i32.const 15160
      call $_ZN4core9panicking5panic17h1f31dfdebac1729fE
      unreachable
    end
    local.get 0)
  (func $_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17hb408146a39027d78E (type 9) (param i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    i32.const 8
    i32.add
    local.get 0
    i32.const 8
    i32.add
    i32.load
    i32.store
    local.get 1
    local.get 0
    i64.load align=4
    i64.store
    local.get 1
    call $_ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17h6269f23e4ae37022E
    unreachable)
  (func $_ZN90_$LT$std..panicking..begin_panic_handler..PanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17hff411efd02c22cd0E (type 2) (param i32 i32)
    (local i32 i32 i32)
    local.get 1
    call $_ZN3std9panicking19begin_panic_handler12PanicPayload4fill17h9af7534c4c371d05E
    local.tee 1
    i32.load offset=8
    local.set 2
    local.get 1
    i32.load offset=4
    local.set 3
    local.get 1
    i64.const 0
    i64.store offset=4 align=4
    local.get 1
    i32.load
    local.set 4
    local.get 1
    i32.const 1
    i32.store
    block  ;; label = @1
      i32.const 12
      i32.const 4
      call $__rust_alloc
      local.tee 1
      br_if 0 (;@1;)
      i32.const 12
      i32.const 4
      call $_ZN5alloc5alloc18handle_alloc_error17h8e32739f1fa5c58cE
      unreachable
    end
    local.get 1
    local.get 2
    i32.store offset=8
    local.get 1
    local.get 3
    i32.store offset=4
    local.get 1
    local.get 4
    i32.store
    local.get 0
    i32.const 15180
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func $_ZN3std9panicking19begin_panic_handler12PanicPayload4fill17h9af7534c4c371d05E (type 4) (param i32) (result i32)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 1
    global.set 0
    local.get 0
    i32.const 4
    i32.add
    local.set 2
    block  ;; label = @1
      local.get 0
      i32.load offset=4
      br_if 0 (;@1;)
      local.get 0
      i32.load
      local.set 3
      local.get 1
      i32.const 0
      i32.store offset=32
      local.get 1
      i64.const 1
      i64.store offset=24
      local.get 1
      local.get 1
      i32.const 24
      i32.add
      i32.store offset=36
      local.get 1
      i32.const 40
      i32.add
      i32.const 16
      i32.add
      local.get 3
      i32.const 16
      i32.add
      i64.load align=4
      i64.store
      local.get 1
      i32.const 40
      i32.add
      i32.const 8
      i32.add
      local.get 3
      i32.const 8
      i32.add
      i64.load align=4
      i64.store
      local.get 1
      local.get 3
      i64.load align=4
      i64.store offset=40
      local.get 1
      i32.const 36
      i32.add
      i32.const 10736
      local.get 1
      i32.const 40
      i32.add
      call $_ZN4core3fmt5write17hbe0dd8e9c0d9aeb7E
      drop
      local.get 1
      i32.const 8
      i32.add
      i32.const 8
      i32.add
      local.tee 3
      local.get 1
      i32.load offset=32
      i32.store
      local.get 1
      local.get 1
      i64.load offset=24
      i64.store offset=8
      block  ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.const 8
        i32.add
        i32.load
        local.tee 0
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        local.get 0
        call $__rust_dealloc
      end
      local.get 2
      local.get 1
      i64.load offset=8
      i64.store align=4
      local.get 2
      i32.const 8
      i32.add
      local.get 3
      i32.load
      i32.store
    end
    local.get 1
    i32.const 64
    i32.add
    global.set 0
    local.get 2)
  (func $_ZN4core3ptr13drop_in_place17h02e1550be636b150E (type 9) (param i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.load offset=4
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load
      local.get 1
      call $__rust_dealloc
    end)
  (func $_ZN90_$LT$std..panicking..begin_panic_handler..PanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$3get17h46310b10c141f08fE (type 2) (param i32 i32)
    local.get 1
    call $_ZN3std9panicking19begin_panic_handler12PanicPayload4fill17h9af7534c4c371d05E
    local.set 1
    local.get 0
    i32.const 15180
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func $_ZN3std9panicking20rust_panic_with_hook17h64cb0accada9ce0eE (type 5) (param i32 i32 i32 i32)
    (local i32 i32)
    i32.const 0
    i32.const 0
    i32.load offset=15308
    i32.const 1
    i32.add
    i32.store offset=15308
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            i32.const 0
            i32.load offset=15312
            local.tee 4
            i32.const 1
            i32.ne
            br_if 0 (;@4;)
            i32.const 15316
            i32.const 0
            local.get 4
            i32.const 1
            i32.eq
            select
            local.tee 4
            local.get 4
            i32.load
            i32.const 1
            i32.add
            local.tee 4
            i32.store
            local.get 4
            i32.const 2
            i32.gt_u
            br_if 3 (;@1;)
            i32.const 0
            i32.load offset=15320
            local.tee 5
            i32.const -1
            i32.gt_s
            br_if 1 (;@3;)
            br 3 (;@1;)
          end
          i32.const 0
          i64.const 1
          i64.store offset=15312
          i32.const 0
          i32.const 1
          i32.store offset=15316
          i32.const 0
          i32.load offset=15320
          local.tee 4
          i32.const 0
          i32.lt_s
          br_if 2 (;@1;)
          i32.const 0
          local.get 4
          i32.store offset=15320
          br 1 (;@2;)
        end
        i32.const 0
        local.get 5
        i32.store offset=15320
        local.get 4
        i32.const 1
        i32.gt_u
        br_if 1 (;@1;)
      end
      call $rust_panic
      unreachable
    end
    unreachable
    unreachable)
  (func $rust_panic (type 6)
    unreachable
    unreachable)
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h2a7b7c33b71febceE (type 12) (param i32) (result i64)
    i64.const 5229932410705260468)
  (func $_ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17h6269f23e4ae37022E (type 9) (param i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    i32.const 15240
    local.get 0
    i32.load offset=4
    i32.load offset=8
    local.get 0
    i32.load offset=8
    call $_ZN3std9panicking20rust_panic_with_hook17h64cb0accada9ce0eE
    unreachable)
  (func $_ZN4core3ptr13drop_in_place17hf1099cb5f23da3a1E (type 9) (param i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.load offset=4
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const 8
      i32.add
      i32.load
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      local.get 0
      call $__rust_dealloc
    end)
  (func $_ZN70_$LT$wee_alloc..LargeAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$22new_cell_for_free_list17hfd780db55d8403d7E (type 8) (param i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 2
        i32.shl
        local.tee 1
        local.get 2
        i32.const 3
        i32.shl
        i32.const 512
        i32.add
        local.tee 2
        local.get 1
        local.get 2
        i32.gt_u
        select
        i32.const 65543
        i32.add
        local.tee 1
        i32.const 16
        i32.shr_u
        memory.grow
        local.tee 2
        i32.const -1
        i32.ne
        br_if 0 (;@2;)
        i32.const 1
        local.set 1
        i32.const 0
        local.set 2
        br 1 (;@1;)
      end
      local.get 2
      i32.const 16
      i32.shl
      local.tee 2
      i64.const 0
      i64.store offset=4 align=4
      local.get 2
      local.get 2
      local.get 1
      i32.const -65536
      i32.and
      i32.add
      i32.const 2
      i32.or
      i32.store
      i32.const 0
      local.set 1
    end
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func $_ZN9wee_alloc15alloc_first_fit17h9bc4bd4c3d54ee85E (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      local.get 2
      i32.load
      local.tee 3
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const -1
      i32.add
      local.set 4
      local.get 0
      i32.const 2
      i32.shl
      local.set 5
      i32.const 0
      local.get 1
      i32.sub
      local.set 6
      loop  ;; label = @2
        local.get 3
        i32.const 8
        i32.add
        local.set 0
        block  ;; label = @3
          local.get 3
          i32.load offset=8
          local.tee 7
          i32.const 1
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          loop  ;; label = @4
            local.get 0
            local.get 7
            i32.const -2
            i32.and
            i32.store
            block  ;; label = @5
              block  ;; label = @6
                local.get 3
                i32.load offset=4
                local.tee 7
                i32.const -4
                i32.and
                local.tee 0
                br_if 0 (;@6;)
                i32.const 0
                local.set 1
                br 1 (;@5;)
              end
              i32.const 0
              local.get 0
              local.get 0
              i32.load8_u
              i32.const 1
              i32.and
              select
              local.set 1
            end
            block  ;; label = @5
              local.get 3
              i32.load
              local.tee 8
              i32.const -4
              i32.and
              local.tee 9
              i32.eqz
              br_if 0 (;@5;)
              i32.const 0
              local.get 9
              local.get 8
              i32.const 2
              i32.and
              select
              local.tee 8
              i32.eqz
              br_if 0 (;@5;)
              local.get 8
              local.get 8
              i32.load offset=4
              i32.const 3
              i32.and
              local.get 0
              i32.or
              i32.store offset=4
              local.get 3
              i32.load offset=4
              local.tee 7
              i32.const -4
              i32.and
              local.set 0
            end
            block  ;; label = @5
              local.get 0
              i32.eqz
              br_if 0 (;@5;)
              local.get 0
              local.get 0
              i32.load
              i32.const 3
              i32.and
              local.get 3
              i32.load
              i32.const -4
              i32.and
              i32.or
              i32.store
              local.get 3
              i32.load offset=4
              local.set 7
            end
            local.get 3
            local.get 7
            i32.const 3
            i32.and
            i32.store offset=4
            local.get 3
            local.get 3
            i32.load
            local.tee 0
            i32.const 3
            i32.and
            i32.store
            block  ;; label = @5
              local.get 0
              i32.const 2
              i32.and
              i32.eqz
              br_if 0 (;@5;)
              local.get 1
              local.get 1
              i32.load
              i32.const 2
              i32.or
              i32.store
            end
            local.get 2
            local.get 1
            i32.store
            local.get 1
            i32.const 8
            i32.add
            local.set 0
            local.get 1
            local.set 3
            local.get 1
            i32.load offset=8
            local.tee 7
            i32.const 1
            i32.and
            br_if 0 (;@4;)
          end
          local.get 1
          local.set 3
        end
        block  ;; label = @3
          local.get 3
          i32.load
          i32.const -4
          i32.and
          local.tee 1
          local.get 0
          i32.sub
          local.get 5
          i32.lt_u
          br_if 0 (;@3;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.const 72
              i32.add
              local.get 1
              local.get 5
              i32.sub
              local.get 6
              i32.and
              local.tee 1
              i32.le_u
              br_if 0 (;@5;)
              local.get 4
              local.get 0
              i32.and
              br_if 2 (;@3;)
              local.get 2
              local.get 7
              i32.const -4
              i32.and
              i32.store
              local.get 3
              local.get 3
              i32.load
              i32.const 1
              i32.or
              i32.store
              local.get 3
              local.set 1
              br 1 (;@4;)
            end
            local.get 1
            i32.const 0
            i32.store
            local.get 1
            i32.const -8
            i32.add
            local.tee 1
            i64.const 0
            i64.store align=4
            local.get 1
            local.get 3
            i32.load
            i32.const -4
            i32.and
            i32.store
            block  ;; label = @5
              local.get 3
              i32.load
              local.tee 7
              i32.const -4
              i32.and
              local.tee 8
              i32.eqz
              br_if 0 (;@5;)
              i32.const 0
              local.get 8
              local.get 7
              i32.const 2
              i32.and
              select
              local.tee 7
              i32.eqz
              br_if 0 (;@5;)
              local.get 7
              local.get 7
              i32.load offset=4
              i32.const 3
              i32.and
              local.get 1
              i32.or
              i32.store offset=4
            end
            local.get 1
            local.get 1
            i32.load offset=4
            i32.const 3
            i32.and
            local.get 3
            i32.or
            i32.store offset=4
            local.get 3
            local.get 3
            i32.load
            i32.const 3
            i32.and
            local.get 1
            i32.or
            i32.store
            local.get 0
            local.get 0
            i32.load
            i32.const -2
            i32.and
            i32.store
            block  ;; label = @5
              local.get 3
              i32.load
              local.tee 0
              i32.const 2
              i32.and
              i32.eqz
              br_if 0 (;@5;)
              local.get 3
              local.get 0
              i32.const -3
              i32.and
              i32.store
              local.get 1
              local.get 1
              i32.load
              i32.const 2
              i32.or
              i32.store
            end
            local.get 1
            local.get 1
            i32.load
            i32.const 1
            i32.or
            i32.store
          end
          local.get 1
          i32.const 8
          i32.add
          return
        end
        local.get 2
        local.get 3
        i32.load offset=8
        local.tee 3
        i32.store
        local.get 3
        br_if 0 (;@2;)
      end
    end
    i32.const 0)
  (func $memcpy (type 0) (param i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.set 3
      loop  ;; label = @2
        local.get 3
        local.get 1
        i32.load8_u
        i32.store8
        local.get 3
        i32.const 1
        i32.add
        local.set 3
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 2
        i32.const -1
        i32.add
        local.tee 2
        br_if 0 (;@2;)
      end
    end
    local.get 0)
  (func $memmove (type 0) (param i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        local.get 0
        i32.lt_u
        br_if 0 (;@2;)
        local.get 2
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        local.set 3
        loop  ;; label = @3
          local.get 3
          local.get 1
          i32.load8_u
          i32.store8
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 3
          i32.const 1
          i32.add
          local.set 3
          local.get 2
          i32.const -1
          i32.add
          local.tee 2
          br_if 0 (;@3;)
          br 2 (;@1;)
        end
      end
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const -1
      i32.add
      local.set 1
      local.get 0
      i32.const -1
      i32.add
      local.set 3
      loop  ;; label = @2
        local.get 3
        local.get 2
        i32.add
        local.get 1
        local.get 2
        i32.add
        i32.load8_u
        i32.store8
        local.get 2
        i32.const -1
        i32.add
        local.tee 2
        br_if 0 (;@2;)
      end
    end
    local.get 0)
  (func $memcmp (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    i32.const 0
    local.set 3
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      block  ;; label = @2
        loop  ;; label = @3
          local.get 0
          i32.load8_u
          local.tee 4
          local.get 1
          i32.load8_u
          local.tee 5
          i32.ne
          br_if 1 (;@2;)
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 0
          i32.const 1
          i32.add
          local.set 0
          local.get 2
          i32.const -1
          i32.add
          local.tee 2
          i32.eqz
          br_if 2 (;@1;)
          br 0 (;@3;)
        end
      end
      local.get 4
      local.get 5
      i32.sub
      local.set 3
    end
    local.get 3)
  (table (;0;) 38 38 funcref)
  (memory (;0;) 1)
  (global (;0;) (mut i32) (i32.const 10240))
  (global (;1;) i32 (i32.const 15328))
  (global (;2;) i32 (i32.const 15328))
  (export "memory" (memory 0))
  (export "main" (func $main))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (elem (;0;) (i32.const 1) func $_ZN74_$LT$unicode_bmp_blocks_selector..BMPBlock$u20$as$u20$core..fmt..Debug$GT$3fmt17h5f69e6c0e6b743a2E $_ZN43_$LT$char$u20$as$u20$core..fmt..Display$GT$3fmt17h21bc6ab835592adfE $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hfcaccb2e0e81eaebE $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h31e3a003075045c9E $_ZN3std5alloc24default_alloc_error_hook17hb03d50b55ebca620E $_ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$usize$GT$3fmt17h60d343ae9ad25d2dE $_ZN4core3ops8function6FnOnce9call_once17h8a477fe68b043fe4E $_ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h768de56d2684a6efE $_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17h72030837644d4392E $_ZN4core3ptr13drop_in_place17h1dffba9b32db7ea5E $_ZN53_$LT$core..fmt..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17hfb3d464953e3231aE $_ZN4core3ptr13drop_in_place17h5842ab0252ffe1ccE $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h29e73825d1f5f372E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h9cff9d77aed7896dE $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17hf1fbebbdcf92fdc7E $_ZN4core3ptr13drop_in_place17h6d8597584bc4ba58E $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17hf545e9d77a58de68E $_ZN4core3fmt5Write10write_char17h9885dd2687cf2480E $_ZN4core3fmt5Write9write_fmt17h3a716cb550662ed4E $_ZN4core3ptr13drop_in_place17hc4ce7ec6fd1fe8d0E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h4650a8e2c1d42a98E $_ZN4core3ptr13drop_in_place17h48010a17ef1ba2d2E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17hd10da3a7c84c86a1E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17ha2b0ac3db39c866aE $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17hd92bcf7983e2e3ccE $_ZN4core3ptr13drop_in_place17hac568a8f4dd0fc8dE $_ZN64_$LT$icu_uniset..UnicodeSetError$u20$as$u20$core..fmt..Debug$GT$3fmt17hf5d167b28bb4dc42E $_ZN4core3ptr13drop_in_place17h74320d6e375583feE $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h997443bf2251431aE $_ZN4core3ptr13drop_in_place17h18f90f79f1b1f11eE.628 $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hde54dc4b6c8fa177E $_ZN4core3ptr13drop_in_place17h74320d6e375583feE.651 $_ZN4core3ptr13drop_in_place17h02e1550be636b150E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h2a7b7c33b71febceE $_ZN4core3ptr13drop_in_place17hf1099cb5f23da3a1E $_ZN90_$LT$std..panicking..begin_panic_handler..PanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17hff411efd02c22cd0E $_ZN90_$LT$std..panicking..begin_panic_handler..PanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$3get17h46310b10c141f08fE)
  (data (;0;) (i32.const 10240) "\00\00\00\00\7f\00\00\00\00\00\00\00\80\00\00\00\ff\00\00\00\00\00\00\00Welcome to MyName\c2\a9\c2\ae, \d0\90\d0\bb\d0\b5\d0\ba\d1\81\d0\b5\d0\b9!\00\00\00\00\00\00 \00\00\00\00\00\00\00\02\00\00\00\00\00\00\00\02\00\00\00\00\00\00\00\03\00\00\00\01\00\00\00 \00\00\00\04\00\00\00\02\00\00\00\00\00\00\00\02\00\00\00\00\00\00\00\03\00\00\00UnknownLatin1SupplementBasiccapacity overflow\00\00\00\c0(\00\00p\00\00\00\1d\02\00\00\05\00\00\00/home/sffc/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec.rsa formatting trait implementation returned an error\00\0a\00\00\00\00\00\00\00\01\00\00\00\0b\00\00\00\84)\00\00l\00\00\00A\02\00\00\1c\00\00\00/home/sffc/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/fmt.rs\0c\00\00\00\04\00\00\00\04\00\00\00\0d\00\00\00\0e\00\00\00\0f\00\00\0000010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899\e2*\00\00o\00\00\00e\00\00\00\14\00\00\000x/home/sffc/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/fmt/num.rs\00\00\00d+\00\00\10\00\00\00t+\00\00\22\00\00\00range end index  out of range for slice of length \00\00\a8+\00\00\16\00\00\00\be+\00\00\0d\00\00\00slice index starts at  but ends at \00\dc+\00\00\12\00\00\00t+\00\00\22\00\00\00range start index \00\00\c4:\00\00\02\00\00\00\88;\00\00\00\00\00\00m8\00\00\02\00\00\00\18,\00\00t\00\00\00Z\00\00\00\05\00\00\00/home/sffc/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/memchr.rs\00\01\03\05\05\06\06\03\07\06\08\08\09\11\0a\1c\0b\19\0c\14\0d\10\0e\0d\0f\04\10\03\12\12\13\09\16\01\17\05\18\02\19\03\1a\07\1c\02\1d\01\1f\16 \03+\03,\02-\0b.\010\031\022\01\a7\02\a9\02\aa\04\ab\08\fa\02\fb\05\fd\04\fe\03\ff\09\adxy\8b\8d\a20WX\8b\8c\90\1c\1d\dd\0e\0fKL\fb\fc./?\5c]_\b5\e2\84\8d\8e\91\92\a9\b1\ba\bb\c5\c6\c9\ca\de\e4\e5\ff\00\04\11\12)147:;=IJ]\84\8e\92\a9\b1\b4\ba\bb\c6\ca\ce\cf\e4\e5\00\04\0d\0e\11\12)14:;EFIJ^de\84\91\9b\9d\c9\ce\cf\0d\11)EIWde\8d\91\a9\b4\ba\bb\c5\c9\df\e4\e5\f0\0d\11EIde\80\84\b2\bc\be\bf\d5\d7\f0\f1\83\85\8b\a4\a6\be\bf\c5\c7\ce\cf\da\dbH\98\bd\cd\c6\ce\cfINOWY^_\89\8e\8f\b1\b6\b7\bf\c1\c6\c7\d7\11\16\17[\5c\f6\f7\fe\ff\80\0dmq\de\df\0e\0f\1fno\1c\1d_}~\ae\af\bb\bc\fa\16\17\1e\1fFGNOXZ\5c^~\7f\b5\c5\d4\d5\dc\f0\f1\f5rs\8ftu\96/_&./\a7\af\b7\bf\c7\cf\d7\df\9a@\97\980\8f\1f\c0\c1\ce\ffNOZ[\07\08\0f\10'/\ee\efno7=?BE\90\91\fe\ffSgu\c8\c9\d0\d1\d8\d9\e7\fe\ff\00 _\22\82\df\04\82D\08\1b\04\06\11\81\ac\0e\80\ab5(\0b\80\e0\03\19\08\01\04/\044\04\07\03\01\07\06\07\11\0aP\0f\12\07U\07\03\04\1c\0a\09\03\08\03\07\03\02\03\03\03\0c\04\05\03\0b\06\01\0e\15\05:\03\11\07\06\05\10\07W\07\02\07\15\0dP\04C\03-\03\01\04\11\06\0f\0c:\04\1d%_ m\04j%\80\c8\05\82\b0\03\1a\06\82\fd\03Y\07\15\0b\17\09\14\0c\14\0cj\06\0a\06\1a\06Y\07+\05F\0a,\04\0c\04\01\031\0b,\04\1a\06\0b\03\80\ac\06\0a\06!?L\04-\03t\08<\03\0f\03<\078\08+\05\82\ff\11\18\08/\11-\03 \10!\0f\80\8c\04\82\97\19\0b\15\88\94\05/\05;\07\02\0e\18\09\80\b3-t\0c\80\d6\1a\0c\05\80\ff\05\80\df\0c\ee\0d\03\84\8d\037\09\81\5c\14\80\b8\08\80\cb*8\03\0a\068\08F\08\0c\06t\0b\1e\03Z\04Y\09\80\83\18\1c\0a\16\09L\04\80\8a\06\ab\a4\0c\17\041\a1\04\81\da&\07\0c\05\05\80\a5\11\81m\10x(*\06L\04\80\8d\04\80\be\03\1b\03\0f\0d\00\06\01\01\03\01\04\02\08\08\09\02\0a\05\0b\02\0e\04\10\01\11\02\12\05\13\11\14\01\15\02\17\02\19\0d\1c\05\1d\08$\01j\03k\02\bc\02\d1\02\d4\0c\d5\09\d6\02\d7\02\da\01\e0\05\e1\02\e8\02\ee \f0\04\f8\02\f9\02\fa\02\fb\01\0c';>NO\8f\9e\9e\9f\06\07\096=>V\f3\d0\d1\04\14\1867VW\7f\aa\ae\af\bd5\e0\12\87\89\8e\9e\04\0d\0e\11\12)14:EFIJNOde\5c\b6\b7\1b\1c\07\08\0a\0b\14\1769:\a8\a9\d8\d9\097\90\91\a8\07\0a;>fi\8f\92o_\ee\efZb\9a\9b'(U\9d\a0\a1\a3\a4\a7\a8\ad\ba\bc\c4\06\0b\0c\15\1d:?EQ\a6\a7\cc\cd\a0\07\19\1a\22%>?\c5\c6\04 #%&(38:HJLPSUVXZ\5c^`cefksx}\7f\8a\a4\aa\af\b0\c0\d0\ae\afy\ccno\93^\22{\05\03\04-\03f\03\01/.\80\82\1d\031\0f\1c\04$\09\1e\05+\05D\04\0e*\80\aa\06$\04$\04(\084\0b\01\80\90\817\09\16\0a\08\80\989\03c\08\090\16\05!\03\1b\05\01@8\04K\05/\04\0a\07\09\07@ '\04\0c\096\03:\05\1a\07\04\0c\07PI73\0d3\07.\08\0a\81&RN(\08*V\1c\14\17\09N\04\1e\0fC\0e\19\07\0a\06H\08'\09u\0b?A*\06;\05\0a\06Q\06\01\05\10\03\05\80\8bb\1eH\08\0a\80\a6^\22E\0b\0a\06\0d\139\07\0a6,\04\10\80\c0<dS\0cH\09\0aFE\1bH\08S\1d9\81\07F\0a\1d\03GI7\03\0e\08\0a\069\07\0a\816\19\80\b7\01\0f2\0d\83\9bfu\0b\80\c4\8a\bc\84/\8f\d1\82G\a1\b9\829\07*\04\02`&\0aF\0a(\05\13\82\b0[eK\049\07\11@\05\0b\02\0e\97\f8\08\84\d6*\09\a2\f7\81\1f1\03\11\04\08\81\8c\89\04k\05\0d\03\09\07\10\93`\80\f6\0as\08n\17F\80\9a\14\0cW\09\19\80\87\81G\03\85B\0f\15\85P+\80\d5-\03\1a\04\02\81p:\05\01\85\00\80\d7)L\04\0a\04\02\83\11DL=\80\c2<\06\01\04U\05\1b4\02\81\0e,\04d\0cV\0a\80\ae8\1d\0d,\04\09\07\02\0e\06\80\9a\83\d8\08\0d\03\0d\03t\0cY\07\0c\14\0c\048\08\0a\06(\08\22N\81T\0c\15\03\03\05\07\09\19\07\07\09\03\0d\07)\80\cb%\0a\84\06\00\f41\00\00y\00\00\00\0a\00\00\00\1c\00\00\00\f41\00\00y\00\00\00\1a\00\00\006\00\00\00/home/sffc/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/unicode/printable.rs\00\00\00\a02\00\00|\00\00\00R\00\00\00>\00\00\00\a02\00\00|\00\00\00K\00\00\00(\00\00\00\a02\00\00|\00\00\00W\00\00\00\16\00\00\00/home/sffc/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/unicode/unicode_data.rs\00\03\00\00\83\04 \00\91\05`\00]\13\a0\00\12\17\a0\1e\0c \e0\1e\ef, +*0\a0+o\a6`,\02\a8\e0,\1e\fb\e0-\00\fe\a05\9e\ff\e05\fd\01a6\01\0a\a16$\0da7\ab\0e\e18/\18!90\1caF\f3\1e\a1J\f0jaNOo\a1N\9d\bc!Oe\d1\e1O\00\da!P\00\e0\e1Q0\e1aS\ec\e2\a1T\d0\e8\e1T \00.U\f0\01\bfU\00p\00\07\00-\01\01\01\02\01\02\01\01H\0b0\15\10\01e\07\02\06\02\02\01\04#\01\1e\1b[\0b:\09\09\01\18\04\01\09\01\03\01\05+\03w\0f\01 7\01\01\01\04\08\04\01\03\07\0a\02\1d\01:\01\01\01\02\04\08\01\09\01\0a\02\1a\01\02\029\01\04\02\04\02\02\03\03\01\1e\02\03\01\0b\029\01\04\05\01\02\04\01\14\02\16\06\01\01:\01\01\02\01\04\08\01\07\03\0a\02\1e\01;\01\01\01\0c\01\09\01(\01\03\019\03\05\03\01\04\07\02\0b\02\1d\01:\01\02\01\02\01\03\01\05\02\07\02\0b\02\1c\029\02\01\01\02\04\08\01\09\01\0a\02\1d\01H\01\04\01\02\03\01\01\08\01Q\01\02\07\0c\08b\01\02\09\0b\06J\02\1b\01\01\01\01\017\0e\01\05\01\02\05\0b\01$\09\01f\04\01\06\01\02\02\02\19\02\04\03\10\04\0d\01\02\02\06\01\0f\01\00\03\00\03\1d\03\1d\02\1e\02@\02\01\07\08\01\02\0b\09\01-\03w\02\22\01v\03\04\02\09\01\06\03\db\02\02\01:\01\01\07\01\01\01\01\02\08\06\0a\02\010\11?\040\07\01\01\05\01(\09\0c\02 \04\02\02\01\038\01\01\02\03\01\01\03:\08\02\02\98\03\01\0d\01\07\04\01\06\01\03\02\c6:\01\05\00\01\c3!\00\03\8d\01` \00\06i\02\00\04\01\0a \02P\02\00\01\03\01\04\01\19\02\05\01\97\02\1a\12\0d\01&\08\19\0b.\030\01\02\04\02\02'\01C\06\02\02\02\02\0c\01\08\01/\013\01\01\03\02\02\05\02\01\01*\02\08\01\ee\01\02\01\04\01\00\01\00\10\10\10\00\02\00\01\e2\01\95\05\00\03\01\02\05\04(\03\04\01\a5\02\00\04\00\02\99\0b\b0\016\0f8\031\04\02\02E\03$\05\01\08>\01\0c\024\09\0a\04\02\01_\03\02\01\01\02\06\01\a0\01\03\08\15\029\02\01\01\01\01\16\01\0e\07\03\05\c3\08\02\03\01\01\17\01Q\01\02\06\01\01\02\01\01\02\01\02\eb\01\02\04\06\02\01\02\1b\02U\08\02\01\01\02j\01\01\01\02\06\01\01e\03\02\04\01\05\00\09\01\02\f5\01\0a\02\01\01\04\01\90\04\02\02\04\01 \0a(\06\02\04\08\01\09\06\02\03.\0d\01\02\00\07\01\06\01\01R\16\02\07\01\02\01\02z\06\03\01\01\02\01\07\01\01H\02\03\01\01\01\00\02\00\05;\07\00\01?\04Q\01\00\02\00\01\01\03\04\05\08\08\02\07\1e\04\94\03\007\042\08\01\0e\01\16\05\01\0f\00\07\01\11\02\07\01\02\01\05\00\07\00\04\00\07m\07\00`\80\f0\00\00\00\00\5c6\00\00o\00\00\00S\04\00\00\11\00\00\00/home/sffc/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/fmt/mod.rs\00\5c6\00\00o\00\00\00]\04\00\00$\00\00\00\10\00\00\00\0c\00\00\00\04\00\00\00\11\00\00\00\12\00\00\00\13\00\00\00Error\00\00\00\14\00\00\00\00\00\00\00\01\00\00\00\15\00\00\00\1c7\00\00 \00\00\00<7\00\00\12\00\00\00index out of bounds: the len is  but the index is     \00\00\f87\00\00s\00\00\00\b0\01\00\00&\00\00\00\847\00\00t\00\00\000\00\00\00!\00\00\00\847\00\00t\00\00\001\00\00\00\12\00\00\00/home/sffc/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/fmt/builders.rs/home/sffc/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/pattern.rs, : ,\0a((\0a,[]\00\16\00\00\00\04\00\00\00\04\00\00\00\17\00\00\00\18\00\00\00\19\00\00\00[...]\00\00\00\f88\00\00\0b\00\00\00Z9\00\00\16\00\00\0079\00\00\01\00\00\0089\00\00\0e\00\00\00F9\00\00\04\00\00\00J9\00\00\10\00\00\0079\00\00\01\00\00\00\f88\00\00\0b\00\00\00\039\00\00&\00\00\00)9\00\00\08\00\00\0019\00\00\06\00\00\0079\00\00\01\00\00\00byte index  is not a char boundary; it is inside  (bytes ) of ``begin <= end ( <= ) when slicing ` is out of bounds of `\809\00\00l\00\00\00%\05\00\00*\00\00\00/home/sffc/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec.rs\1a\00\00\00\10\00\00\00\04\00\00\00\1b\00\00\00\0c:\00\00 \00\00\00\18\00\00\009\00\00\00components/uniset/src/builder.rs\0c:\00\00 \00\00\00-\00\00\00\11\00\00\00\0c:\00\00 \00\00\005\00\00\00\15\00\00\00\5c:\00\00\1f\00\00\00-\00\00\00#\00\00\00components/uniset/src/uniset.rsInvalidRange\00\1c\00\00\00\04\00\00\00\04\00\00\00\1d\00\00\00InvalidSet\00\00\1e\00\00\00\04\00\00\00\04\00\00\00\1f\00\00\00 \00\00\00\04\00\00\00\04\00\00\00\1d\00\00\00../home/sffc/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs\00\00\c6:\00\00p\00\00\00\e2\01\00\00\1e\00\00\00\0a\00\00\00!\00\00\00\0c\00\00\00\04\00\00\00\22\00\00\00)called `Option::unwrap()` on a `None` value#\00\00\00\10\00\00\00\04\00\00\00$\00\00\00%\00\00\00called `Result::unwrap()` on an `Err` value"))
