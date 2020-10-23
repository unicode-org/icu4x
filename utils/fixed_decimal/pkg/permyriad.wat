(module
  (type (;0;) (func (param i32 i32)))
  (type (;1;) (func (param i32 i32 i32 i32)))
  (type (;2;) (func (param i32 i32) (result i32)))
  (type (;3;) (func (param i32) (result i32)))
  (type (;4;) (func (param i32 i32 i32) (result i32)))
  (type (;5;) (func (param i32 i32 i32)))
  (type (;6;) (func))
  (type (;7;) (func (param i32)))
  (type (;8;) (func (param i32) (result i64)))
  (type (;9;) (func (param i32 i32 i32 i32 i32) (result i32)))
  (func $__rg_alloc (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17h8dd94c46d18c9882E)
  (func $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17h8dd94c46d18c9882E (type 2) (param i32 i32) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const 3
      i32.add
      i32.const 2
      i32.shr_u
      local.set 0
      block  ;; label = @2
        local.get 1
        i32.const 4
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const -1
        i32.add
        local.tee 3
        i32.const 255
        i32.gt_u
        br_if 0 (;@2;)
        local.get 2
        i32.const 10480
        i32.store offset=4
        local.get 2
        local.get 3
        i32.const 2
        i32.shl
        i32.const 10484
        i32.add
        local.tee 3
        i32.load
        i32.store offset=12
        local.get 0
        local.get 1
        local.get 2
        i32.const 12
        i32.add
        local.get 2
        i32.const 4
        i32.add
        i32.const 10408
        call $_ZN9wee_alloc17alloc_with_refill17h813e7cd4a6b57740E
        local.set 1
        local.get 3
        local.get 2
        i32.load offset=12
        i32.store
        br 1 (;@1;)
      end
      local.get 2
      i32.const 0
      i32.load offset=10480
      i32.store offset=8
      local.get 0
      local.get 1
      local.get 2
      i32.const 8
      i32.add
      i32.const 10456
      i32.const 10432
      call $_ZN9wee_alloc17alloc_with_refill17h813e7cd4a6b57740E
      local.set 1
      i32.const 0
      local.get 2
      i32.load offset=8
      i32.store offset=10480
    end
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 1)
  (func $__rg_dealloc (type 0) (param i32 i32)
    local.get 0
    local.get 1
    call $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17hbc89d828dc72ab94E)
  (func $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17hbc89d828dc72ab94E (type 0) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      local.get 0
      i32.store offset=4
      block  ;; label = @2
        local.get 1
        i32.const 3
        i32.add
        i32.const 2
        i32.shr_u
        i32.const -1
        i32.add
        local.tee 0
        i32.const 255
        i32.gt_u
        br_if 0 (;@2;)
        local.get 2
        i32.const 10480
        i32.store offset=8
        local.get 2
        local.get 0
        i32.const 2
        i32.shl
        i32.const 10484
        i32.add
        local.tee 0
        i32.load
        i32.store offset=12
        local.get 2
        i32.const 4
        i32.add
        local.get 2
        i32.const 12
        i32.add
        local.get 2
        i32.const 8
        i32.add
        i32.const 10408
        call $_ZN9wee_alloc8WeeAlloc12dealloc_impl28_$u7b$$u7b$closure$u7d$$u7d$17h07ec904e08402f48E
        local.get 0
        local.get 2
        i32.load offset=12
        i32.store
        br 1 (;@1;)
      end
      local.get 2
      i32.const 0
      i32.load offset=10480
      i32.store offset=12
      local.get 2
      i32.const 4
      i32.add
      local.get 2
      i32.const 12
      i32.add
      i32.const 10456
      i32.const 10432
      call $_ZN9wee_alloc8WeeAlloc12dealloc_impl28_$u7b$$u7b$closure$u7d$$u7d$17h07ec904e08402f48E
      i32.const 0
      local.get 2
      i32.load offset=12
      i32.store offset=10480
    end
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func $__rg_realloc (type 4) (param i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      local.get 2
      i32.const 1
      call $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17h8dd94c46d18c9882E
      local.tee 3
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      local.get 0
      local.get 2
      local.get 1
      local.get 1
      local.get 2
      i32.gt_u
      select
      call $memcpy
      drop
      local.get 0
      local.get 1
      call $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17hbc89d828dc72ab94E
    end
    local.get 3)
  (func $main (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 27
    i32.add
    i32.const 0
    i32.store align=1
    local.get 2
    i32.const 0
    i32.store offset=24
    local.get 2
    i32.const 0
    i32.store8 offset=12
    local.get 2
    i32.const 0
    i32.store offset=8
    block  ;; label = @1
      i32.const 1
      i32.const 1
      call $__rust_alloc
      local.tee 3
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      i64.const 1
      i64.store offset=36 align=4
      local.get 2
      local.get 3
      i32.store offset=32
      i32.const 48
      local.set 4
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.load16_s offset=24
          local.tee 3
          i32.const 0
          i32.lt_s
          br_if 0 (;@3;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              i32.load offset=8
              local.tee 5
              i32.const 8
              i32.gt_u
              br_if 0 (;@5;)
              local.get 2
              i32.const 8
              i32.add
              i32.const 5
              i32.or
              local.set 6
              br 1 (;@4;)
            end
            local.get 2
            i32.load offset=20
            local.set 5
            local.get 2
            i32.load offset=16
            local.set 6
          end
          local.get 6
          i32.eqz
          br_if 0 (;@3;)
          local.get 5
          local.get 3
          i32.le_u
          br_if 0 (;@3;)
          local.get 6
          local.get 3
          i32.add
          i32.load8_u
          i32.const 24
          i32.shl
          i32.const 805306368
          i32.add
          i32.const 24
          i32.shr_s
          local.tee 4
          i32.const -1
          i32.gt_s
          br_if 0 (;@3;)
          local.get 2
          i32.const 0
          i32.store offset=44
          local.get 2
          local.get 4
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=45
          local.get 2
          local.get 4
          i32.const 192
          i32.and
          i32.const 6
          i32.shr_u
          i32.const 192
          i32.or
          i32.store8 offset=44
          local.get 2
          i32.const 32
          i32.add
          local.get 2
          i32.const 44
          i32.add
          i32.const 2
          call $_ZN5alloc3vec12Vec$LT$T$GT$17extend_from_slice17hef5121037079282aE
          br 1 (;@2;)
        end
        block  ;; label = @3
          local.get 2
          i32.load offset=40
          local.tee 3
          local.get 2
          i32.load offset=36
          i32.ne
          br_if 0 (;@3;)
          local.get 2
          i32.const 32
          i32.add
          local.get 3
          i32.const 1
          call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve17he48c34cb4402aee3E
          local.get 2
          i32.load offset=40
          local.set 3
        end
        local.get 2
        i32.load offset=32
        local.get 3
        i32.add
        local.get 4
        i32.store8
        local.get 2
        local.get 3
        i32.const 1
        i32.add
        i32.store offset=40
      end
      block  ;; label = @2
        local.get 2
        i32.load offset=36
        local.tee 3
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        i32.load offset=32
        local.get 3
        call $__rust_dealloc
      end
      block  ;; label = @2
        local.get 2
        i32.load offset=8
        local.tee 3
        i32.const 8
        i32.le_u
        br_if 0 (;@2;)
        local.get 2
        i32.load offset=16
        local.get 3
        call $__rust_dealloc
      end
      local.get 2
      i32.const 48
      i32.add
      global.set 0
      i32.const 0
      return
    end
    i32.const 1
    i32.const 1
    call $_ZN5alloc5alloc18handle_alloc_error17hf2282e1fd09e2a06E
    unreachable)
  (func $__rust_alloc (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $__rg_alloc)
  (func $_ZN5alloc3vec12Vec$LT$T$GT$17extend_from_slice17hef5121037079282aE (type 5) (param i32 i32 i32)
    local.get 0
    local.get 0
    i32.load offset=8
    local.get 2
    call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve17he48c34cb4402aee3E
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
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve17he48c34cb4402aee3E (type 5) (param i32 i32 i32)
    (local i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 3
        local.get 1
        i32.sub
        local.get 2
        i32.ge_u
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              local.get 2
              i32.add
              local.tee 2
              local.get 1
              i32.lt_u
              br_if 0 (;@5;)
              i32.const 0
              local.set 4
              local.get 3
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
              local.tee 2
              i32.const 0
              i32.lt_s
              br_if 4 (;@1;)
              local.get 0
              i32.load
              i32.const 0
              local.get 3
              select
              local.tee 1
              i32.eqz
              br_if 1 (;@4;)
              local.get 3
              i32.eqz
              br_if 1 (;@4;)
              local.get 1
              local.get 3
              local.get 2
              call $__rust_realloc
              local.tee 1
              br_if 2 (;@3;)
              i32.const 1
              local.set 4
              br 4 (;@1;)
            end
            i32.const 0
            local.set 4
            br 3 (;@1;)
          end
          i32.const 1
          local.set 4
          local.get 2
          i32.const 1
          call $__rust_alloc
          local.tee 1
          i32.eqz
          br_if 2 (;@1;)
        end
        local.get 0
        local.get 2
        i32.store offset=4
        local.get 0
        local.get 1
        i32.store
      end
      return
    end
    block  ;; label = @1
      local.get 4
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      local.get 4
      call $_ZN5alloc5alloc18handle_alloc_error17hf2282e1fd09e2a06E
      unreachable
    end
    call $_ZN5alloc7raw_vec17capacity_overflow17h49dfdde6764ef78dE
    unreachable)
  (func $__rust_dealloc (type 0) (param i32 i32)
    local.get 0
    local.get 1
    call $__rg_dealloc)
  (func $_ZN5alloc5alloc18handle_alloc_error17hf2282e1fd09e2a06E (type 0) (param i32 i32)
    local.get 0
    local.get 1
    call $__rust_alloc_error_handler
    unreachable)
  (func $__rust_realloc (type 4) (param i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    call $__rg_realloc)
  (func $__rust_alloc_error_handler (type 0) (param i32 i32)
    local.get 0
    local.get 1
    call $__rg_oom
    unreachable)
  (func $__rg_oom (type 0) (param i32 i32)
    (local i32)
    local.get 0
    local.get 1
    i32.const 0
    i32.load offset=11524
    local.tee 2
    i32.const 1
    local.get 2
    select
    call_indirect (type 0)
    unreachable
    unreachable)
  (func $_ZN3std5alloc24default_alloc_error_hook17hfc9fa99831a390b0E (type 0) (param i32 i32))
  (func $_ZN5alloc7raw_vec17capacity_overflow17h49dfdde6764ef78dE (type 6)
    i32.const 10240
    i32.const 17
    i32.const 10260
    call $_ZN4core9panicking5panic17hc886a4cb4479b06eE
    unreachable)
  (func $_ZN4core9panicking5panic17hc886a4cb4479b06eE (type 5) (param i32 i32 i32)
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
    i32.const 10456
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
    call $_ZN4core9panicking9panic_fmt17h6aa2a8f84484b5f7E
    unreachable)
  (func $_ZN4core9panicking9panic_fmt17h6aa2a8f84484b5f7E (type 0) (param i32 i32)
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
    i32.const 10304
    i32.store offset=4
    local.get 2
    i32.const 10456
    i32.store
    local.get 2
    call $rust_begin_unwind
    unreachable)
  (func $rust_begin_unwind (type 7) (param i32)
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
    call $_ZN4core6option15Option$LT$T$GT$6unwrap17h77da55fec473aad9E
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
    call $_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17haaf70eaaf969e47fE
    unreachable)
  (func $_ZN4core3ptr13drop_in_place17h00c08aab80423b88E (type 7) (param i32))
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h128e23c99f6446a5E (type 8) (param i32) (result i64)
    i64.const 5966890128770411197)
  (func $_ZN3std9panicking20rust_panic_with_hook17h380371049f7f28a2E (type 6)
    (local i32 i32)
    i32.const 0
    i32.const 0
    i32.load offset=11508
    i32.const 1
    i32.add
    i32.store offset=11508
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          i32.const 0
          i32.load offset=11512
          i32.const 1
          i32.ne
          br_if 0 (;@3;)
          i32.const 0
          i32.const 0
          i32.load offset=11516
          i32.const 1
          i32.add
          local.tee 0
          i32.store offset=11516
          local.get 0
          i32.const 2
          i32.gt_u
          br_if 2 (;@1;)
          i32.const 0
          i32.load offset=11520
          local.tee 1
          i32.const -1
          i32.gt_s
          br_if 1 (;@2;)
          br 2 (;@1;)
        end
        i32.const 0
        i64.const 4294967297
        i64.store offset=11512
        i32.const 0
        i32.load offset=11520
        local.tee 0
        i32.const 0
        i32.lt_s
        br_if 1 (;@1;)
        i32.const 0
        local.get 0
        i32.store offset=11520
        call $rust_panic
        unreachable
      end
      i32.const 0
      local.get 1
      i32.store offset=11520
      local.get 0
      i32.const 1
      i32.gt_u
      br_if 0 (;@1;)
      call $rust_panic
      unreachable
    end
    unreachable
    unreachable)
  (func $rust_panic (type 6)
    unreachable
    unreachable)
  (func $_ZN4core6option15Option$LT$T$GT$6unwrap17h77da55fec473aad9E (type 3) (param i32) (result i32)
    block  ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      i32.const 10320
      i32.const 43
      i32.const 10364
      call $_ZN4core9panicking5panic17hc886a4cb4479b06eE
      unreachable
    end
    local.get 0)
  (func $_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17haaf70eaaf969e47fE (type 7) (param i32)
    local.get 0
    i32.load
    call $_ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17hf32ac274ad929b2eE
    unreachable)
  (func $_ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17hf32ac274ad929b2eE (type 7) (param i32)
    call $_ZN3std9panicking20rust_panic_with_hook17h380371049f7f28a2E
    unreachable)
  (func $_ZN70_$LT$wee_alloc..LargeAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$22new_cell_for_free_list17h6bdae789a7603eb8E (type 1) (param i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.const 2
        i32.shl
        local.tee 2
        local.get 3
        i32.const 3
        i32.shl
        i32.const 16384
        i32.add
        local.tee 3
        local.get 2
        local.get 3
        i32.gt_u
        select
        i32.const 65543
        i32.add
        local.tee 2
        i32.const 16
        i32.shr_u
        memory.grow
        local.tee 3
        i32.const -1
        i32.ne
        br_if 0 (;@2;)
        i32.const 1
        local.set 2
        i32.const 0
        local.set 3
        br 1 (;@1;)
      end
      local.get 3
      i32.const 16
      i32.shl
      local.tee 3
      i64.const 0
      i64.store offset=4 align=4
      local.get 3
      local.get 3
      local.get 2
      i32.const -65536
      i32.and
      i32.add
      i32.const 2
      i32.or
      i32.store
      i32.const 0
      local.set 2
    end
    local.get 0
    local.get 3
    i32.store offset=4
    local.get 0
    local.get 2
    i32.store)
  (func $_ZN70_$LT$wee_alloc..LargeAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$13min_cell_size17h39c4479e998ccdc2E (type 2) (param i32 i32) (result i32)
    i32.const 512)
  (func $_ZN70_$LT$wee_alloc..LargeAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$32should_merge_adjacent_free_cells17h9faf2a075fc1e8d9E (type 3) (param i32) (result i32)
    i32.const 1)
  (func $_ZN9wee_alloc17alloc_with_refill17h813e7cd4a6b57740E (type 9) (param i32 i32 i32 i32 i32) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 5
    global.set 0
    block  ;; label = @1
      local.get 0
      local.get 1
      local.get 2
      local.get 3
      local.get 4
      call $_ZN9wee_alloc15alloc_first_fit17hc681025884bb691fE
      local.tee 6
      br_if 0 (;@1;)
      local.get 5
      i32.const 8
      i32.add
      local.get 3
      local.get 0
      local.get 1
      local.get 4
      i32.load offset=12
      call_indirect (type 1)
      i32.const 0
      local.set 6
      local.get 5
      i32.load offset=8
      br_if 0 (;@1;)
      local.get 5
      i32.load offset=12
      local.tee 6
      local.get 2
      i32.load
      i32.store offset=8
      local.get 2
      local.get 6
      i32.store
      local.get 0
      local.get 1
      local.get 2
      local.get 3
      local.get 4
      call $_ZN9wee_alloc15alloc_first_fit17hc681025884bb691fE
      local.set 6
    end
    local.get 5
    i32.const 16
    i32.add
    global.set 0
    local.get 6)
  (func $_ZN9wee_alloc15alloc_first_fit17hc681025884bb691fE (type 9) (param i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      local.get 2
      i32.load
      local.tee 5
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const -1
      i32.add
      local.set 6
      local.get 0
      i32.const 2
      i32.shl
      local.set 7
      i32.const 0
      local.get 1
      i32.sub
      local.set 8
      loop  ;; label = @2
        local.get 5
        i32.const 8
        i32.add
        local.set 9
        block  ;; label = @3
          local.get 5
          i32.load offset=8
          local.tee 10
          i32.const 1
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          loop  ;; label = @4
            local.get 9
            local.get 10
            i32.const -2
            i32.and
            i32.store
            block  ;; label = @5
              block  ;; label = @6
                local.get 5
                i32.load offset=4
                local.tee 10
                i32.const -4
                i32.and
                local.tee 9
                br_if 0 (;@6;)
                i32.const 0
                local.set 1
                br 1 (;@5;)
              end
              i32.const 0
              local.get 9
              local.get 9
              i32.load8_u
              i32.const 1
              i32.and
              select
              local.set 1
            end
            block  ;; label = @5
              local.get 5
              i32.load
              local.tee 11
              i32.const -4
              i32.and
              local.tee 12
              i32.eqz
              br_if 0 (;@5;)
              i32.const 0
              local.get 12
              local.get 11
              i32.const 2
              i32.and
              select
              local.tee 11
              i32.eqz
              br_if 0 (;@5;)
              local.get 11
              local.get 11
              i32.load offset=4
              i32.const 3
              i32.and
              local.get 9
              i32.or
              i32.store offset=4
              local.get 5
              i32.load offset=4
              local.tee 10
              i32.const -4
              i32.and
              local.set 9
            end
            block  ;; label = @5
              local.get 9
              i32.eqz
              br_if 0 (;@5;)
              local.get 9
              local.get 9
              i32.load
              i32.const 3
              i32.and
              local.get 5
              i32.load
              i32.const -4
              i32.and
              i32.or
              i32.store
              local.get 5
              i32.load offset=4
              local.set 10
            end
            local.get 5
            local.get 10
            i32.const 3
            i32.and
            i32.store offset=4
            local.get 5
            local.get 5
            i32.load
            local.tee 9
            i32.const 3
            i32.and
            i32.store
            block  ;; label = @5
              local.get 9
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
            local.set 9
            local.get 1
            local.set 5
            local.get 1
            i32.load offset=8
            local.tee 10
            i32.const 1
            i32.and
            br_if 0 (;@4;)
          end
          local.get 1
          local.set 5
        end
        block  ;; label = @3
          local.get 5
          i32.load
          i32.const -4
          i32.and
          local.tee 1
          local.get 9
          i32.sub
          local.get 7
          i32.lt_u
          br_if 0 (;@3;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 9
              local.get 3
              local.get 0
              local.get 4
              i32.load offset=16
              call_indirect (type 2)
              i32.const 2
              i32.shl
              i32.add
              i32.const 8
              i32.add
              local.get 1
              local.get 7
              i32.sub
              local.get 8
              i32.and
              local.tee 1
              i32.le_u
              br_if 0 (;@5;)
              local.get 6
              local.get 9
              i32.and
              br_if 2 (;@3;)
              local.get 2
              local.get 9
              i32.load
              i32.const -4
              i32.and
              i32.store
              local.get 5
              local.get 5
              i32.load
              i32.const 1
              i32.or
              i32.store
              local.get 5
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
            local.get 5
            i32.load
            i32.const -4
            i32.and
            i32.store
            block  ;; label = @5
              local.get 5
              i32.load
              local.tee 10
              i32.const -4
              i32.and
              local.tee 11
              i32.eqz
              br_if 0 (;@5;)
              i32.const 0
              local.get 11
              local.get 10
              i32.const 2
              i32.and
              select
              local.tee 10
              i32.eqz
              br_if 0 (;@5;)
              local.get 10
              local.get 10
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
            local.get 5
            i32.or
            i32.store offset=4
            local.get 5
            local.get 5
            i32.load
            i32.const 3
            i32.and
            local.get 1
            i32.or
            i32.store
            local.get 9
            local.get 9
            i32.load
            i32.const -2
            i32.and
            i32.store
            block  ;; label = @5
              local.get 5
              i32.load
              local.tee 9
              i32.const 2
              i32.and
              i32.eqz
              br_if 0 (;@5;)
              local.get 5
              local.get 9
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
        local.get 5
        i32.load offset=8
        local.tee 5
        i32.store
        local.get 5
        br_if 0 (;@2;)
      end
    end
    i32.const 0)
  (func $_ZN4core3ptr13drop_in_place17h35855da05c919d6cE (type 7) (param i32))
  (func $_ZN4core3ptr13drop_in_place17h3a39399158ea62dbE (type 7) (param i32))
  (func $_ZN9wee_alloc8WeeAlloc12dealloc_impl28_$u7b$$u7b$closure$u7d$$u7d$17h07ec904e08402f48E (type 1) (param i32 i32 i32 i32)
    (local i32 i32 i32)
    local.get 0
    i32.load
    local.tee 4
    i32.const 0
    i32.store
    local.get 4
    i32.const -8
    i32.add
    local.tee 0
    local.get 0
    i32.load
    i32.const -2
    i32.and
    i32.store
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        local.get 3
        i32.load offset=20
        call_indirect (type 3)
        i32.eqz
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            local.get 4
            i32.const -4
            i32.add
            local.tee 3
            i32.load
            i32.const -4
            i32.and
            local.tee 2
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.load
            local.tee 5
            i32.const 1
            i32.and
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 0
          i32.load
          local.tee 2
          i32.const -4
          i32.and
          local.tee 3
          i32.eqz
          br_if 1 (;@2;)
          i32.const 0
          local.get 3
          local.get 2
          i32.const 2
          i32.and
          select
          local.tee 2
          i32.eqz
          br_if 1 (;@2;)
          local.get 2
          i32.load8_u
          i32.const 1
          i32.and
          br_if 1 (;@2;)
          local.get 4
          local.get 2
          i32.load offset=8
          i32.const -4
          i32.and
          i32.store
          local.get 2
          local.get 0
          i32.const 1
          i32.or
          i32.store offset=8
          return
        end
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.load
              local.tee 6
              i32.const -4
              i32.and
              local.tee 4
              br_if 0 (;@5;)
              local.get 2
              local.set 1
              br 1 (;@4;)
            end
            local.get 2
            local.set 1
            i32.const 0
            local.get 4
            local.get 6
            i32.const 2
            i32.and
            select
            local.tee 6
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            local.get 6
            i32.load offset=4
            i32.const 3
            i32.and
            local.get 2
            i32.or
            i32.store offset=4
            local.get 3
            i32.load
            local.tee 4
            i32.const -4
            i32.and
            local.tee 1
            i32.eqz
            br_if 1 (;@3;)
            local.get 0
            i32.load
            i32.const -4
            i32.and
            local.set 4
            local.get 1
            i32.load
            local.set 5
          end
          local.get 1
          local.get 5
          i32.const 3
          i32.and
          local.get 4
          i32.or
          i32.store
          local.get 3
          i32.load
          local.set 4
        end
        local.get 3
        local.get 4
        i32.const 3
        i32.and
        i32.store
        local.get 0
        local.get 0
        i32.load
        local.tee 4
        i32.const 3
        i32.and
        i32.store
        local.get 4
        i32.const 2
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 2
        local.get 2
        i32.load
        i32.const 2
        i32.or
        i32.store
        return
      end
      local.get 4
      local.get 1
      i32.load
      i32.store
      local.get 1
      local.get 0
      i32.store
    end)
  (func $_ZN88_$LT$wee_alloc..size_classes..SizeClassAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$22new_cell_for_free_list17h61b50fe9b628b9faE (type 1) (param i32 i32 i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 4
    global.set 0
    local.get 4
    local.get 1
    i32.load
    local.tee 1
    i32.load
    i32.store offset=12
    local.get 2
    i32.const 2
    i32.add
    local.tee 2
    local.get 2
    i32.mul
    local.tee 2
    i32.const 2048
    local.get 2
    i32.const 2048
    i32.gt_u
    select
    local.tee 5
    i32.const 4
    local.get 4
    i32.const 12
    i32.add
    i32.const 10456
    i32.const 10456
    call $_ZN9wee_alloc17alloc_with_refill17h813e7cd4a6b57740E
    local.set 2
    local.get 1
    local.get 4
    i32.load offset=12
    i32.store
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        br_if 0 (;@2;)
        i32.const 1
        local.set 1
        br 1 (;@1;)
      end
      local.get 2
      i64.const 0
      i64.store offset=4 align=4
      local.get 2
      local.get 2
      local.get 5
      i32.const 2
      i32.shl
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
    i32.store
    local.get 4
    i32.const 16
    i32.add
    global.set 0)
  (func $_ZN88_$LT$wee_alloc..size_classes..SizeClassAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$13min_cell_size17hb9a928dad4aa0158E (type 2) (param i32 i32) (result i32)
    local.get 1)
  (func $_ZN88_$LT$wee_alloc..size_classes..SizeClassAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$32should_merge_adjacent_free_cells17hfb91ab08d962f28cE (type 3) (param i32) (result i32)
    i32.const 0)
  (func $_ZN4core3ptr13drop_in_place17h35855da05c919d6cE.257 (type 7) (param i32))
  (func $memcpy (type 4) (param i32 i32 i32) (result i32)
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
        br_if 0 (;@2;)
      end
    end
    local.get 0)
  (table (;0;) 13 13 funcref)
  (memory (;0;) 1)
  (global (;0;) (mut i32) (i32.const 10240))
  (global (;1;) i32 (i32.const 11528))
  (global (;2;) i32 (i32.const 11528))
  (export "memory" (memory 0))
  (export "main" (func $main))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (elem (;0;) (i32.const 1) func $_ZN3std5alloc24default_alloc_error_hook17hfc9fa99831a390b0E $_ZN4core3ptr13drop_in_place17h00c08aab80423b88E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h128e23c99f6446a5E $_ZN4core3ptr13drop_in_place17h3a39399158ea62dbE $_ZN88_$LT$wee_alloc..size_classes..SizeClassAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$22new_cell_for_free_list17h61b50fe9b628b9faE $_ZN88_$LT$wee_alloc..size_classes..SizeClassAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$13min_cell_size17hb9a928dad4aa0158E $_ZN88_$LT$wee_alloc..size_classes..SizeClassAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$32should_merge_adjacent_free_cells17hfb91ab08d962f28cE $_ZN4core3ptr13drop_in_place17h35855da05c919d6cE $_ZN70_$LT$wee_alloc..LargeAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$22new_cell_for_free_list17h6bdae789a7603eb8E $_ZN70_$LT$wee_alloc..LargeAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$13min_cell_size17h39c4479e998ccdc2E $_ZN70_$LT$wee_alloc..LargeAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$32should_merge_adjacent_free_cells17h9faf2a075fc1e8d9E $_ZN4core3ptr13drop_in_place17h35855da05c919d6cE.257)
  (data (;0;) (i32.const 10240) "capacity overflow\00\00\00$(\00\00\1c\00\00\00\1d\02\00\00\05\00\00\00library/alloc/src/raw_vec.rs\02\00\00\00\00\00\00\00\01\00\00\00\03\00\00\00called `Option::unwrap()` on a `None` value\00\8c(\00\00\1c\00\00\00\e2\01\00\00\1e\00\00\00library/std/src/panicking.rs\04\00\00\00\04\00\00\00\04\00\00\00\05\00\00\00\06\00\00\00\07\00\00\00\08\00\00\00\00\00\00\00\01\00\00\00\09\00\00\00\0a\00\00\00\0b\00\00\00\0c\00\00\00\00\00\00\00\01\00\00\00\09\00\00\00\0a\00\00\00\0b\00\00\00"))
