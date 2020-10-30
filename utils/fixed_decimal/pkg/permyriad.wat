(module
  (type (;0;) (func (param i32 i32)))
  (type (;1;) (func (param i32 i32 i32) (result i32)))
  (type (;2;) (func (param i32 i32) (result i32)))
  (type (;3;) (func (param i32 i32 i32 i32)))
  (type (;4;) (func (param i32) (result i32)))
  (type (;5;) (func (param i32 i32 i32)))
  (type (;6;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;7;) (func))
  (type (;8;) (func (param i32)))
  (type (;9;) (func (param i32) (result i64)))
  (type (;10;) (func (param i64 i32 i32) (result i32)))
  (type (;11;) (func (param i32 i32 i32 i32 i32 i32) (result i32)))
  (type (;12;) (func (param i32 i32 i32 i32 i32) (result i32)))
  (func $__rg_alloc (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17h01592a764b68148eE)
  (func $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17h01592a764b68148eE (type 2) (param i32 i32) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 0
    i32.const 3
    i32.add
    i32.const 2
    i32.shr_u
    local.set 0
    block  ;; label = @1
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
        i32.const 11120
        i32.store offset=4
        local.get 2
        local.get 3
        i32.const 2
        i32.shl
        i32.const 11124
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
        i32.const 11048
        call $_ZN9wee_alloc17alloc_with_refill17hc6d059644918d278E
        local.set 1
        local.get 3
        local.get 2
        i32.load offset=12
        i32.store
        br 1 (;@1;)
      end
      local.get 2
      i32.const 0
      i32.load offset=11120
      i32.store offset=8
      local.get 0
      local.get 1
      local.get 2
      i32.const 8
      i32.add
      i32.const 11096
      i32.const 11072
      call $_ZN9wee_alloc17alloc_with_refill17hc6d059644918d278E
      local.set 1
      i32.const 0
      local.get 2
      i32.load offset=8
      i32.store offset=11120
    end
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 1)
  (func $__rg_dealloc (type 5) (param i32 i32 i32)
    local.get 0
    local.get 1
    local.get 2
    call $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17ha5bc9b3a43b818b2E)
  (func $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17ha5bc9b3a43b818b2E (type 5) (param i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    block  ;; label = @1
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      local.get 0
      i32.store offset=4
      block  ;; label = @2
        local.get 2
        i32.const 4
        i32.gt_u
        br_if 0 (;@2;)
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
        local.get 3
        i32.const 11120
        i32.store offset=8
        local.get 3
        local.get 0
        i32.const 2
        i32.shl
        i32.const 11124
        i32.add
        local.tee 0
        i32.load
        i32.store offset=12
        local.get 3
        i32.const 4
        i32.add
        local.get 3
        i32.const 12
        i32.add
        local.get 3
        i32.const 8
        i32.add
        i32.const 11048
        call $_ZN9wee_alloc8WeeAlloc12dealloc_impl28_$u7b$$u7b$closure$u7d$$u7d$17h21395ae90c962d08E
        local.get 0
        local.get 3
        i32.load offset=12
        i32.store
        br 1 (;@1;)
      end
      local.get 3
      i32.const 0
      i32.load offset=11120
      i32.store offset=12
      local.get 3
      i32.const 4
      i32.add
      local.get 3
      i32.const 12
      i32.add
      i32.const 11096
      i32.const 11072
      call $_ZN9wee_alloc8WeeAlloc12dealloc_impl28_$u7b$$u7b$closure$u7d$$u7d$17h21395ae90c962d08E
      i32.const 0
      local.get 3
      i32.load offset=12
      i32.store offset=11120
    end
    local.get 3
    i32.const 16
    i32.add
    global.set 0)
  (func $__rg_realloc (type 6) (param i32 i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      local.get 3
      local.get 2
      call $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17h01592a764b68148eE
      local.tee 4
      i32.eqz
      br_if 0 (;@1;)
      local.get 4
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
      local.get 2
      call $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17ha5bc9b3a43b818b2E
    end
    local.get 4)
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
    local.get 2
    i32.const 1
    call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in17hec4a77debe4148f8E
    local.get 2
    i32.const 0
    i32.store offset=40
    local.get 2
    local.get 2
    i64.load
    i64.store offset=32
    i32.const 48
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.load16_s offset=24
        local.tee 4
        i32.const 0
        i32.lt_s
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.load offset=8
            local.tee 5
            i32.const 8
            i32.gt_u
            br_if 0 (;@4;)
            local.get 2
            i32.const 8
            i32.add
            i32.const 5
            i32.or
            local.set 6
            br 1 (;@3;)
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
        br_if 0 (;@2;)
        local.get 5
        local.get 4
        i32.le_u
        br_if 0 (;@2;)
        local.get 6
        local.get 4
        i32.add
        i32.load8_u
        i32.const 24
        i32.shl
        i32.const 805306368
        i32.add
        i32.const 24
        i32.shr_s
        local.tee 3
        i32.const -1
        i32.gt_s
        br_if 0 (;@2;)
        local.get 2
        i32.const 0
        i32.store offset=44
        local.get 2
        local.get 3
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=45
        local.get 2
        local.get 3
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
        local.get 2
        i32.const 44
        i32.add
        i32.const 2
        i32.or
        call $_ZN115_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..SpecExtend$LT$$RF$T$C$core..slice..iter..Iter$LT$T$GT$$GT$$GT$11spec_extend17hdc6d8e4cac9df68cE
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 2
        i32.load offset=40
        local.tee 4
        local.get 2
        i32.load offset=36
        i32.ne
        br_if 0 (;@2;)
        local.get 2
        i32.const 32
        i32.add
        local.get 4
        i32.const 1
        call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve17h36b4998143ff41efE
        local.get 2
        i32.load offset=40
        local.set 4
      end
      local.get 2
      i32.load offset=32
      local.get 4
      i32.add
      local.get 3
      i32.store8
      local.get 2
      local.get 4
      i32.const 1
      i32.add
      i32.store offset=40
    end
    block  ;; label = @1
      local.get 2
      i32.load offset=36
      local.tee 4
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      i32.load offset=32
      local.get 4
      i32.const 1
      call $__rust_dealloc
    end
    block  ;; label = @1
      local.get 2
      i32.load offset=8
      local.tee 4
      i32.const 8
      i32.le_u
      br_if 0 (;@1;)
      local.get 2
      i32.load offset=16
      local.get 4
      i32.const 1
      call $__rust_dealloc
    end
    local.get 2
    i32.const 48
    i32.add
    global.set 0
    i32.const 0)
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in17hec4a77debe4148f8E (type 0) (param i32 i32)
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
      call $_ZN5alloc7raw_vec17capacity_overflow17h6f470473614799a0E
      unreachable
    end
    local.get 1
    i32.const 1
    call $_ZN5alloc5alloc18handle_alloc_error17hca26941e53fa543fE
    unreachable)
  (func $_ZN115_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..SpecExtend$LT$$RF$T$C$core..slice..iter..Iter$LT$T$GT$$GT$$GT$11spec_extend17hdc6d8e4cac9df68cE (type 5) (param i32 i32 i32)
    local.get 0
    local.get 0
    i32.load offset=8
    local.get 2
    local.get 1
    i32.sub
    local.tee 2
    call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve17h36b4998143ff41efE
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
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve17h36b4998143ff41efE (type 5) (param i32 i32 i32)
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
          call $_ZN5alloc7raw_vec11finish_grow17h2e7719e8c87df8dcE
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
      call $_ZN5alloc5alloc18handle_alloc_error17hca26941e53fa543fE
      unreachable
    end
    call $_ZN5alloc7raw_vec17capacity_overflow17h6f470473614799a0E
    unreachable)
  (func $__rust_dealloc (type 5) (param i32 i32 i32)
    local.get 0
    local.get 1
    local.get 2
    call $__rg_dealloc)
  (func $__rust_alloc (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $__rg_alloc)
  (func $__rust_realloc (type 6) (param i32 i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
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
    i32.load offset=12164
    local.tee 2
    i32.const 1
    local.get 2
    select
    call_indirect (type 0)
    unreachable
    unreachable)
  (func $_ZN5alloc7raw_vec17capacity_overflow17h6f470473614799a0E (type 7)
    i32.const 10240
    i32.const 17
    i32.const 10260
    call $_ZN4core9panicking5panic17h059bb8b4b17f4cbcE
    unreachable)
  (func $_ZN5alloc5alloc18handle_alloc_error17hca26941e53fa543fE (type 0) (param i32 i32)
    local.get 0
    local.get 1
    call $__rust_alloc_error_handler
    unreachable)
  (func $_ZN4core9panicking5panic17h059bb8b4b17f4cbcE (type 5) (param i32 i32 i32)
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
    i32.const 11096
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
    call $_ZN4core9panicking9panic_fmt17hd8ce56461509210eE
    unreachable)
  (func $_ZN5alloc7raw_vec11finish_grow17h2e7719e8c87df8dcE (type 3) (param i32 i32 i32 i32)
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
  (func $_ZN3std5alloc24default_alloc_error_hook17hd578be0a27d40dc1E (type 0) (param i32 i32))
  (func $_ZN4core3ptr13drop_in_place17h87bbcd4b96a429d5E (type 8) (param i32))
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h9cf4a47d4d9fd837E (type 1) (param i32 i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    local.get 1
    local.get 2
    i32.add
    call $_ZN115_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..SpecExtend$LT$$RF$T$C$core..slice..iter..Iter$LT$T$GT$$GT$$GT$11spec_extend17hdc6d8e4cac9df68cE
    i32.const 0)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h4cc14ef41aaead3dE (type 2) (param i32 i32) (result i32)
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
            call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve17h36b4998143ff41efE
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
      call $_ZN115_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..SpecExtend$LT$$RF$T$C$core..slice..iter..Iter$LT$T$GT$$GT$$GT$11spec_extend17hdc6d8e4cac9df68cE
    end
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    i32.const 0)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17hb7ccc1a26ce6fe4fE (type 2) (param i32 i32) (result i32)
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
    i32.const 10388
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17hcdc919081fb76c8fE
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set 0
    local.get 1)
  (func $_ZN4core3fmt5write17hcdc919081fb76c8fE (type 1) (param i32 i32 i32) (result i32)
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
              call_indirect (type 1)
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
              call $_ZN4core3fmt8getcount17h861ec173eab72a8dE
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
              call $_ZN4core3fmt8getcount17h861ec173eab72a8dE
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
                  call_indirect (type 2)
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
                i32.const 10612
                call $_ZN4core9panicking18panic_bounds_check17h9a071d3fa3c05f2aE
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
            call_indirect (type 1)
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
            call_indirect (type 2)
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
          call_indirect (type 1)
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
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h49a75cc037b1108bE (type 9) (param i32) (result i64)
    i64.const -4656704778008813959)
  (func $_ZN4core3fmt3num3imp7fmt_u6417h02426c6ea33fa8b1E (type 10) (param i64 i32 i32) (result i32)
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
        i32.const 10412
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
        i32.const 10412
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
      i32.const 10412
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
        i32.const 10412
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
    i32.const 11096
    i32.const 0
    local.get 3
    i32.const 9
    i32.add
    local.get 4
    i32.add
    i32.const 39
    local.get 4
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17ha732429d695233c6E
    local.set 4
    local.get 3
    i32.const 48
    i32.add
    global.set 0
    local.get 4)
  (func $_ZN4core3fmt9Formatter12pad_integral17ha732429d695233c6E (type 11) (param i32 i32 i32 i32 i32 i32) (result i32)
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
        call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17ha37ce68ff7005af6E
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
        call_indirect (type 1)
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
        call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17ha37ce68ff7005af6E
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
        call_indirect (type 1)
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
            call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17ha37ce68ff7005af6E
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
            call $_ZN4core3fmt9Formatter7padding17h116b7e093af958f1E
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
            call_indirect (type 1)
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
              call_indirect (type 2)
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
          call $_ZN4core3fmt9Formatter7padding17h116b7e093af958f1E
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
          call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17ha37ce68ff7005af6E
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
          call_indirect (type 1)
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
            call_indirect (type 2)
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
  (func $_ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$usize$GT$3fmt17hf44a8e3177939d82E (type 2) (param i32 i32) (result i32)
    local.get 0
    i64.load32_u
    i32.const 1
    local.get 1
    call $_ZN4core3fmt3num3imp7fmt_u6417h02426c6ea33fa8b1E)
  (func $_ZN4core3ops8function6FnOnce9call_once17hb79636bb9f7c0d68E (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    drop
    loop (result i32)  ;; label = @1
      br 0 (;@1;)
    end)
  (func $_ZN4core3fmt8getcount17h861ec173eab72a8dE (type 3) (param i32 i32 i32 i32)
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
          i32.const 10740
          call $_ZN4core9panicking18panic_bounds_check17h9a071d3fa3c05f2aE
          unreachable
        end
        local.get 1
        local.get 3
        i32.const 3
        i32.shl
        i32.add
        local.tee 3
        i32.load offset=4
        i32.const 2
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
  (func $_ZN4core9panicking18panic_bounds_check17h9a071d3fa3c05f2aE (type 5) (param i32 i32 i32)
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
    i32.const 3
    i32.store
    local.get 3
    i64.const 2
    i64.store offset=12 align=4
    local.get 3
    i32.const 10772
    i32.store offset=8
    local.get 3
    i32.const 3
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
    call $_ZN4core9panicking9panic_fmt17hd8ce56461509210eE
    unreachable)
  (func $_ZN4core3fmt9Formatter12pad_integral12write_prefix17ha37ce68ff7005af6E (type 6) (param i32 i32 i32 i32) (result i32)
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
        call_indirect (type 2)
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
      call_indirect (type 1)
      local.set 4
    end
    local.get 4)
  (func $_ZN4core3fmt9Formatter7padding17h116b7e093af958f1E (type 3) (param i32 i32 i32 i32)
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
        call_indirect (type 2)
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
  (func $_ZN4core9panicking9panic_fmt17hd8ce56461509210eE (type 0) (param i32 i32)
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
    i32.const 10756
    i32.store offset=4
    local.get 2
    i32.const 11096
    i32.store
    local.get 2
    call $rust_begin_unwind
    unreachable)
  (func $rust_begin_unwind (type 8) (param i32)
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
    call $_ZN4core6option15Option$LT$T$GT$6unwrap17h7ddf780fab261b67E
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
    call $_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17hf2fbe36ee3d88c65E
    unreachable)
  (func $_ZN4core3ptr13drop_in_place17hf34fc32dfd6a1dcaE (type 8) (param i32))
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hfda674a93a89c2d6E (type 9) (param i32) (result i64)
    i64.const 7504629734305773520)
  (func $_ZN4core6option15Option$LT$T$GT$6unwrap17h7ddf780fab261b67E (type 4) (param i32) (result i32)
    block  ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      i32.const 10984
      i32.const 43
      i32.const 10952
      call $_ZN4core9panicking5panic17h059bb8b4b17f4cbcE
      unreachable
    end
    local.get 0)
  (func $_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17hf2fbe36ee3d88c65E (type 8) (param i32)
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
    call $_ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17h3c9230bae5689cecE
    unreachable)
  (func $_ZN90_$LT$std..panicking..begin_panic_handler..PanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17h5ce115b899cede78E (type 0) (param i32 i32)
    (local i32 i32 i32)
    local.get 1
    call $_ZN3std9panicking19begin_panic_handler12PanicPayload4fill17hd039868956b928dfE
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
      call $_ZN5alloc5alloc18handle_alloc_error17hca26941e53fa543fE
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
    i32.const 10968
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func $_ZN3std9panicking19begin_panic_handler12PanicPayload4fill17hd039868956b928dfE (type 4) (param i32) (result i32)
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
      i32.const 10388
      local.get 1
      i32.const 40
      i32.add
      call $_ZN4core3fmt5write17hcdc919081fb76c8fE
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
        i32.const 1
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
  (func $_ZN4core3ptr13drop_in_place17h4d8018b625cf8376E (type 8) (param i32)
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
      i32.const 1
      call $__rust_dealloc
    end)
  (func $_ZN90_$LT$std..panicking..begin_panic_handler..PanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$3get17h23de12b735da4a26E (type 0) (param i32 i32)
    local.get 1
    call $_ZN3std9panicking19begin_panic_handler12PanicPayload4fill17hd039868956b928dfE
    local.set 1
    local.get 0
    i32.const 10968
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func $_ZN3std9panicking20rust_panic_with_hook17h9f68644f5b7d5017E (type 3) (param i32 i32 i32 i32)
    (local i32 i32)
    i32.const 0
    i32.const 0
    i32.load offset=12148
    i32.const 1
    i32.add
    i32.store offset=12148
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            i32.const 0
            i32.load offset=12152
            local.tee 4
            i32.const 1
            i32.ne
            br_if 0 (;@4;)
            i32.const 12156
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
            i32.load offset=12160
            local.tee 5
            i32.const -1
            i32.gt_s
            br_if 1 (;@3;)
            br 3 (;@1;)
          end
          i32.const 0
          i64.const 1
          i64.store offset=12152
          i32.const 0
          i32.const 1
          i32.store offset=12156
          i32.const 0
          i32.load offset=12160
          local.tee 4
          i32.const 0
          i32.lt_s
          br_if 2 (;@1;)
          i32.const 0
          local.get 4
          i32.store offset=12160
          br 1 (;@2;)
        end
        i32.const 0
        local.get 5
        i32.store offset=12160
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
  (func $rust_panic (type 7)
    unreachable
    unreachable)
  (func $_ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17h3c9230bae5689cecE (type 8) (param i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    i32.const 11028
    local.get 0
    i32.load offset=4
    i32.load offset=8
    local.get 0
    i32.load offset=8
    call $_ZN3std9panicking20rust_panic_with_hook17h9f68644f5b7d5017E
    unreachable)
  (func $_ZN4core3ptr13drop_in_place17h208bb2259968f776E (type 8) (param i32)
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
      i32.const 1
      call $__rust_dealloc
    end)
  (func $_ZN70_$LT$wee_alloc..LargeAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$22new_cell_for_free_list17h6d450bc70398a1ceE (type 3) (param i32 i32 i32 i32)
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
  (func $_ZN70_$LT$wee_alloc..LargeAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$13min_cell_size17h3e09d24d511c162fE (type 2) (param i32 i32) (result i32)
    i32.const 512)
  (func $_ZN70_$LT$wee_alloc..LargeAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$32should_merge_adjacent_free_cells17h0ff09b2be183c512E (type 4) (param i32) (result i32)
    i32.const 1)
  (func $_ZN9wee_alloc17alloc_with_refill17hc6d059644918d278E (type 12) (param i32 i32 i32 i32 i32) (result i32)
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
      call $_ZN9wee_alloc15alloc_first_fit17hec0f0f54bfd81038E
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
      call_indirect (type 3)
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
      call $_ZN9wee_alloc15alloc_first_fit17hec0f0f54bfd81038E
      local.set 6
    end
    local.get 5
    i32.const 16
    i32.add
    global.set 0
    local.get 6)
  (func $_ZN9wee_alloc15alloc_first_fit17hec0f0f54bfd81038E (type 12) (param i32 i32 i32 i32 i32) (result i32)
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
  (func $_ZN4core3ptr13drop_in_place17hfb7c3c3096059d0fE (type 8) (param i32))
  (func $_ZN4core3ptr13drop_in_place17h8480c48a13eaff95E (type 8) (param i32))
  (func $_ZN9wee_alloc8WeeAlloc12dealloc_impl28_$u7b$$u7b$closure$u7d$$u7d$17h21395ae90c962d08E (type 3) (param i32 i32 i32 i32)
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
        call_indirect (type 4)
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
  (func $_ZN88_$LT$wee_alloc..size_classes..SizeClassAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$22new_cell_for_free_list17hc422e17381578f77E (type 3) (param i32 i32 i32 i32)
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
    i32.const 11096
    i32.const 11096
    call $_ZN9wee_alloc17alloc_with_refill17hc6d059644918d278E
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
  (func $_ZN88_$LT$wee_alloc..size_classes..SizeClassAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$13min_cell_size17heac91d6b149faca8E (type 2) (param i32 i32) (result i32)
    local.get 1)
  (func $_ZN88_$LT$wee_alloc..size_classes..SizeClassAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$32should_merge_adjacent_free_cells17h32f5670d323b8a84E (type 4) (param i32) (result i32)
    i32.const 0)
  (func $_ZN4core3ptr13drop_in_place17hfb7c3c3096059d0fE.1622 (type 8) (param i32))
  (func $memcpy (type 1) (param i32 i32 i32) (result i32)
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
  (table (;0;) 24 24 funcref)
  (memory (;0;) 1)
  (global (;0;) (mut i32) (i32.const 10240))
  (global (;1;) i32 (i32.const 12168))
  (global (;2;) i32 (i32.const 12168))
  (export "memory" (memory 0))
  (export "main" (func $main))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (elem (;0;) (i32.const 1) func $_ZN3std5alloc24default_alloc_error_hook17hd578be0a27d40dc1E $_ZN4core3ops8function6FnOnce9call_once17hb79636bb9f7c0d68E $_ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$usize$GT$3fmt17hf44a8e3177939d82E $_ZN4core3ptr13drop_in_place17h87bbcd4b96a429d5E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h9cf4a47d4d9fd837E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h4cc14ef41aaead3dE $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17hb7ccc1a26ce6fe4fE $_ZN4core3ptr13drop_in_place17hf34fc32dfd6a1dcaE $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h49a75cc037b1108bE $_ZN4core3ptr13drop_in_place17h4d8018b625cf8376E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hfda674a93a89c2d6E $_ZN4core3ptr13drop_in_place17h208bb2259968f776E $_ZN90_$LT$std..panicking..begin_panic_handler..PanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17h5ce115b899cede78E $_ZN90_$LT$std..panicking..begin_panic_handler..PanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$3get17h23de12b735da4a26E $_ZN4core3ptr13drop_in_place17h8480c48a13eaff95E $_ZN88_$LT$wee_alloc..size_classes..SizeClassAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$22new_cell_for_free_list17hc422e17381578f77E $_ZN88_$LT$wee_alloc..size_classes..SizeClassAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$13min_cell_size17heac91d6b149faca8E $_ZN88_$LT$wee_alloc..size_classes..SizeClassAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$32should_merge_adjacent_free_cells17h32f5670d323b8a84E $_ZN4core3ptr13drop_in_place17hfb7c3c3096059d0fE $_ZN70_$LT$wee_alloc..LargeAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$22new_cell_for_free_list17h6d450bc70398a1ceE $_ZN70_$LT$wee_alloc..LargeAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$13min_cell_size17h3e09d24d511c162fE $_ZN70_$LT$wee_alloc..LargeAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$32should_merge_adjacent_free_cells17h0ff09b2be183c512E $_ZN4core3ptr13drop_in_place17hfb7c3c3096059d0fE.1622)
  (data (;0;) (i32.const 10240) "capacity overflow\00\00\00$(\00\00p\00\00\00\1d\02\00\00\05\00\00\00/home/sffc/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec.rs\04\00\00\00\04\00\00\00\04\00\00\00\05\00\00\00\06\00\00\00\07\00\00\0000010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899\84)\00\00o\00\00\00S\04\00\00\11\00\00\00/home/sffc/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/fmt/mod.rs\00\84)\00\00o\00\00\00]\04\00\00$\00\00\00\08\00\00\00\00\00\00\00\01\00\00\00\09\00\00\00$*\00\00 \00\00\00D*\00\00\12\00\00\00index out of bounds: the len is  but the index is /home/sffc/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs\00\00V*\00\00p\00\00\00\e2\01\00\00\1e\00\00\00\0a\00\00\00\0c\00\00\00\04\00\00\00\0b\00\00\00called `Option::unwrap()` on a `None` value\00\0c\00\00\00\10\00\00\00\04\00\00\00\0d\00\00\00\0e\00\00\00\0f\00\00\00\04\00\00\00\04\00\00\00\10\00\00\00\11\00\00\00\12\00\00\00\13\00\00\00\00\00\00\00\01\00\00\00\14\00\00\00\15\00\00\00\16\00\00\00\17\00\00\00\00\00\00\00\01\00\00\00\14\00\00\00\15\00\00\00\16\00\00\00"))
