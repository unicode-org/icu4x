(module
  (type (;0;) (func (param i32 i32)))
  (type (;1;) (func (param i32 i32 i32) (result i32)))
  (type (;2;) (func (param i32 i32) (result i32)))
  (type (;3;) (func (param i32 i32 i32 i32)))
  (type (;4;) (func (param i32) (result i32)))
  (type (;5;) (func (param i32 i32 i32)))
  (type (;6;) (func (param i32)))
  (type (;7;) (func (param i32) (result i64)))
  (type (;8;) (func (param i32 i32 i32 i32 i32) (result i32)))
  (type (;9;) (func (param i32 i32)))
  (type (;10;) (func (param i32 i32 i32) (result i32)))
  (type (;11;) (func (param i32)))
  (type (;12;) (func (param i32 i32 i32)))
  (type (;13;) (func (param i32 i32) (result i32)))
  (type (;14;) (func (param i64 i32) (result i32)))
  (func (;0;) (type 2) (param i32 i32) (result i32)
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
        call 40
        local.set 1
        local.get 3
        local.get 2
        i32.load offset=12
        i32.store
        br 1 (;@1;)
      end
      local.get 2
      i32.const 11120
      i32.load
      i32.store offset=8
      local.get 0
      local.get 1
      local.get 2
      i32.const 8
      i32.add
      i32.const 11096
      i32.const 11072
      call 40
      local.set 1
      i32.const 11120
      local.get 2
      i32.load offset=8
      i32.store
    end
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 1)
  (func (;1;) (type 9) (param i32 i32)
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
      local.get 1
      i32.const 3
      i32.add
      i32.const 2
      i32.shr_u
      i32.const -1
      i32.add
      local.tee 0
      i32.const 255
      i32.le_u
      if  ;; label = @2
        local.get 2
        i32.const 11120
        i32.store offset=8
        local.get 2
        local.get 0
        i32.const 2
        i32.shl
        i32.const 11124
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
        i32.const 11048
        call 42
        local.get 0
        local.get 2
        i32.load offset=12
        i32.store
        br 1 (;@1;)
      end
      local.get 2
      i32.const 11120
      i32.load
      i32.store offset=12
      local.get 2
      i32.const 4
      i32.add
      local.get 2
      i32.const 12
      i32.add
      i32.const 11096
      i32.const 11072
      call 42
      i32.const 11120
      local.get 2
      i32.load offset=12
      i32.store
    end
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func (;2;) (type 10) (param i32 i32 i32) (result i32)
    (local i32)
    local.get 2
    i32.const 1
    call 0
    local.tee 3
    if  ;; label = @1
      local.get 3
      local.get 0
      local.get 2
      local.get 1
      local.get 1
      local.get 2
      i32.gt_u
      select
      call 46
      local.get 0
      local.get 1
      call 1
    end
    local.get 3)
  (func (;3;) (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 0
    global.set 0
    local.get 0
    i32.const 27
    i32.add
    i32.const 0
    i32.store align=1
    local.get 0
    i32.const 0
    i32.store offset=24
    local.get 0
    i32.const 0
    i32.store8 offset=12
    local.get 0
    i32.const 0
    i32.store offset=8
    local.get 0
    call 4
    local.get 0
    i32.const 0
    i32.store offset=40
    local.get 0
    local.get 0
    i64.load
    i64.store offset=32
    i32.const 48
    local.set 1
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load16_s offset=24
        local.tee 3
        i32.const 0
        i32.lt_s
        br_if 0 (;@2;)
        block (result i32)  ;; label = @3
          local.get 0
          i32.const 8
          i32.add
          i32.const 5
          i32.or
          local.get 0
          i32.load offset=8
          local.tee 4
          i32.const 8
          i32.le_u
          br_if 0 (;@3;)
          drop
          local.get 0
          i32.load offset=20
          local.set 4
          local.get 0
          i32.load offset=16
        end
        local.tee 2
        i32.eqz
        local.get 4
        local.get 3
        i32.le_u
        i32.or
        br_if 0 (;@2;)
        local.get 2
        local.get 3
        i32.add
        i32.load8_u
        i32.const 24
        i32.shl
        i32.const 805306368
        i32.add
        i32.const 24
        i32.shr_s
        local.tee 1
        i32.const -1
        i32.gt_s
        br_if 0 (;@2;)
        local.get 0
        i32.const 0
        i32.store offset=44
        local.get 0
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=45
        local.get 0
        local.get 1
        i32.const 192
        i32.and
        i32.const 6
        i32.shr_u
        i32.const 192
        i32.or
        i32.store8 offset=44
        local.get 0
        i32.const 32
        i32.add
        local.get 0
        i32.const 44
        i32.add
        local.get 0
        i32.const 44
        i32.add
        i32.const 2
        i32.or
        call 5
        br 1 (;@1;)
      end
      local.get 0
      i32.load offset=40
      local.tee 2
      local.get 0
      i32.load offset=36
      i32.eq
      if  ;; label = @2
        local.get 0
        i32.const 32
        i32.add
        local.get 2
        i32.const 1
        call 6
        local.get 0
        i32.load offset=40
        local.set 2
      end
      local.get 0
      i32.load offset=32
      local.get 2
      i32.add
      local.get 1
      i32.store8
      local.get 0
      local.get 2
      i32.const 1
      i32.add
      i32.store offset=40
    end
    local.get 0
    i32.load offset=36
    local.tee 1
    if  ;; label = @1
      local.get 0
      i32.load offset=32
      local.get 1
      call 7
    end
    local.get 0
    i32.load offset=8
    local.tee 1
    i32.const 8
    i32.gt_u
    if  ;; label = @1
      local.get 0
      i32.load offset=16
      local.get 1
      call 7
    end
    local.get 0
    i32.const 48
    i32.add
    global.set 0
    i32.const 0)
  (func (;4;) (type 11) (param i32)
    (local i32)
    i32.const 1
    i32.const 1
    call 8
    local.tee 1
    if  ;; label = @1
      local.get 0
      i32.const 1
      i32.store offset=4
      local.get 0
      local.get 1
      i32.store
      return
    end
    i32.const 1
    i32.const 1
    call 9
    unreachable)
  (func (;5;) (type 5) (param i32 i32 i32)
    local.get 0
    local.get 0
    i32.load offset=8
    local.get 2
    local.get 1
    i32.sub
    local.tee 2
    call 6
    local.get 0
    i32.load
    local.get 0
    i32.load offset=8
    i32.add
    local.get 1
    local.get 2
    call 46
    local.get 0
    local.get 0
    i32.load offset=8
    local.get 2
    i32.add
    i32.store offset=8)
  (func (;6;) (type 5) (param i32 i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 4
        local.get 1
        i32.sub
        local.get 2
        i32.lt_u
        if  ;; label = @3
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
            local.get 4
            if  ;; label = @5
              local.get 3
              i32.const 24
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
          local.get 3
          i32.const 16
          i32.add
          call 11
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
      call 9
      unreachable
    end
    i32.const 10240
    i32.const 17
    i32.const 10260
    call 10
    unreachable)
  (func (;7;) (type 9) (param i32 i32)
    local.get 0
    local.get 1
    call 1)
  (func (;8;) (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call 0)
  (func (;9;) (type 0) (param i32 i32)
    local.get 0
    local.get 1
    i32.const 12164
    i32.load
    local.tee 0
    i32.const 1
    local.get 0
    select
    call_indirect (type 0)
    unreachable)
  (func (;10;) (type 5) (param i32 i32 i32)
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
    call 27
    unreachable)
  (func (;11;) (type 12) (param i32 i32 i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      block (result i32)  ;; label = @2
        local.get 1
        i32.const 0
        i32.lt_s
        br_if 1 (;@1;)
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 2
                  i32.load
                  local.tee 3
                  i32.eqz
                  if  ;; label = @8
                    local.get 1
                    i32.eqz
                    br_if 1 (;@7;)
                    br 3 (;@5;)
                  end
                  local.get 2
                  i32.load offset=4
                  local.tee 2
                  br_if 1 (;@6;)
                  local.get 1
                  br_if 2 (;@5;)
                end
                i32.const 1
                local.set 2
                br 3 (;@3;)
              end
              local.get 3
              local.get 2
              local.get 1
              call 2
              local.tee 2
              i32.eqz
              br_if 1 (;@4;)
              br 2 (;@3;)
            end
            local.get 1
            i32.const 1
            call 8
            local.tee 2
            br_if 1 (;@3;)
          end
          local.get 0
          local.get 1
          i32.store offset=4
          i32.const 1
          local.set 1
          i32.const 1
          br 1 (;@2;)
        end
        local.get 0
        local.get 2
        i32.store offset=4
        i32.const 0
      end
      i32.store
      local.get 0
      i32.const 8
      i32.add
      local.get 1
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
    i32.store)
  (func (;12;) (type 0) (param i32 i32)
    nop)
  (func (;13;) (type 6) (param i32)
    nop)
  (func (;14;) (type 1) (param i32 i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    local.get 1
    local.get 2
    i32.add
    call 5
    i32.const 0)
  (func (;15;) (type 2) (param i32 i32) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 0
      local.get 2
      i32.const 12
      i32.add
      block (result i32)  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.const 128
          i32.ge_u
          if  ;; label = @4
            local.get 2
            i32.const 0
            i32.store offset=12
            local.get 1
            i32.const 2048
            i32.lt_u
            br_if 1 (;@3;)
            local.get 1
            i32.const 65536
            i32.lt_u
            if  ;; label = @5
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
            br 2 (;@2;)
          end
          local.get 0
          i32.load offset=8
          local.tee 3
          local.get 0
          i32.load offset=4
          i32.eq
          if (result i32)  ;; label = @4
            local.get 0
            local.get 3
            i32.const 1
            call 6
            local.get 0
            i32.load offset=8
          else
            local.get 3
          end
          local.get 0
          i32.load
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
      end
      local.get 2
      i32.const 12
      i32.add
      i32.add
      call 5
    end
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    i32.const 0)
  (func (;16;) (type 2) (param i32 i32) (result i32)
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
    i32.const 24
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 16
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
    local.get 2
    i32.const 8
    i32.add
    call 17
    local.get 2
    i32.const 32
    i32.add
    global.set 0)
  (func (;17;) (type 13) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
    i32.const 10388
    local.set 6
    global.get 0
    i32.const -64
    i32.add
    local.tee 2
    global.set 0
    local.get 2
    i32.const 52
    i32.add
    i32.const 10388
    i32.store
    local.get 2
    i32.const 3
    i32.store8 offset=56
    local.get 2
    i64.const 137438953472
    i64.store offset=24
    local.get 2
    local.get 0
    i32.store offset=48
    local.get 2
    i32.const 0
    i32.store offset=40
    local.get 2
    i32.const 0
    i32.store offset=32
    block (result i32)  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.load offset=8
          local.tee 3
          if  ;; label = @4
            local.get 1
            i32.load
            local.set 7
            local.get 1
            i32.load offset=4
            local.tee 8
            local.get 1
            i32.const 12
            i32.add
            i32.load
            i32.const 134217727
            i32.and
            local.tee 5
            local.get 5
            local.get 8
            i32.gt_u
            select
            local.tee 5
            i32.eqz
            br_if 1 (;@3;)
            local.get 5
            i32.const 3
            i32.shl
            i32.const -8
            i32.add
            local.set 10
            loop  ;; label = @5
              local.get 0
              local.get 4
              local.get 7
              i32.add
              local.tee 0
              i32.load
              local.get 0
              i32.const 4
              i32.add
              i32.load
              local.get 6
              i32.load offset=12
              call_indirect (type 1)
              br_if 3 (;@2;)
              local.get 2
              local.get 3
              i32.const 28
              i32.add
              i32.load8_u
              i32.store8 offset=56
              local.get 2
              local.get 3
              i32.const 4
              i32.add
              i64.load align=4
              i64.const 32
              i64.rotl
              i64.store offset=24
              local.get 2
              i32.const 16
              i32.add
              local.get 1
              i32.load offset=16
              local.tee 6
              local.get 1
              i32.load offset=20
              local.tee 0
              local.get 3
              i32.const 20
              i32.add
              call 23
              local.get 2
              local.get 2
              i64.load offset=16
              i64.store offset=32
              local.get 2
              i32.const 8
              i32.add
              local.get 6
              local.get 0
              local.get 3
              i32.const 12
              i32.add
              call 23
              local.get 2
              local.get 2
              i64.load offset=8
              i64.store offset=40
              block  ;; label = @6
                local.get 3
                i32.load
                local.tee 9
                local.get 0
                i32.lt_u
                if  ;; label = @7
                  local.get 6
                  local.get 9
                  i32.const 3
                  i32.shl
                  i32.add
                  local.tee 0
                  i32.load
                  local.get 2
                  i32.const 24
                  i32.add
                  local.get 0
                  i32.load offset=4
                  call_indirect (type 2)
                  br_if 5 (;@2;)
                  local.get 4
                  local.get 10
                  i32.ne
                  br_if 1 (;@6;)
                  local.get 5
                  local.set 4
                  br 4 (;@3;)
                end
                local.get 9
                local.get 0
                i32.const 10612
                call 24
                unreachable
              end
              local.get 3
              i32.const 32
              i32.add
              local.set 3
              local.get 4
              i32.const 8
              i32.add
              local.set 4
              local.get 2
              i32.load offset=52
              local.set 6
              local.get 2
              i32.load offset=48
              local.set 0
              br 0 (;@5;)
            end
            unreachable
          end
          local.get 1
          i32.load
          local.set 7
          local.get 1
          i32.load offset=4
          local.tee 8
          local.get 1
          i32.const 20
          i32.add
          i32.load
          local.tee 5
          local.get 5
          local.get 8
          i32.gt_u
          select
          local.tee 5
          i32.eqz
          br_if 0 (;@3;)
          local.get 5
          i32.const -1
          i32.add
          local.set 4
          local.get 1
          i32.load offset=16
          local.set 1
          i32.const 0
          local.set 3
          loop (result i32)  ;; label = @4
            local.get 0
            local.get 3
            local.get 7
            i32.add
            local.tee 0
            i32.load
            local.get 0
            i32.const 4
            i32.add
            i32.load
            local.get 6
            i32.load offset=12
            call_indirect (type 1)
            br_if 2 (;@2;)
            local.get 1
            local.get 3
            i32.add
            local.tee 0
            i32.load
            local.get 2
            i32.const 24
            i32.add
            local.get 0
            i32.const 4
            i32.add
            i32.load
            call_indirect (type 2)
            br_if 2 (;@2;)
            local.get 4
            if (result i32)  ;; label = @5
              local.get 4
              i32.const -1
              i32.add
              local.set 4
              local.get 3
              i32.const 8
              i32.add
              local.set 3
              local.get 2
              i32.load offset=52
              local.set 6
              local.get 2
              i32.load offset=48
              local.set 0
              br 1 (;@4;)
            else
              local.get 5
            end
          end
          local.set 4
        end
        local.get 8
        local.get 4
        i32.gt_u
        if  ;; label = @3
          local.get 2
          i32.load offset=48
          local.get 7
          local.get 4
          i32.const 3
          i32.shl
          i32.add
          local.tee 0
          i32.load
          local.get 0
          i32.load offset=4
          local.get 2
          i32.load offset=52
          i32.load offset=12
          call_indirect (type 1)
          br_if 1 (;@2;)
        end
        i32.const 0
        br 1 (;@1;)
      end
      i32.const 1
    end
    local.get 2
    i32.const -64
    i32.sub
    global.set 0)
  (func (;18;) (type 7) (param i32) (result i64)
    i64.const -4656704778008813959)
  (func (;19;) (type 14) (param i64 i32) (result i32)
    (local i32 i32 i32 i32 i64)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 4
    global.set 0
    i32.const 39
    local.set 2
    block  ;; label = @1
      local.get 0
      i64.const 10000
      i64.lt_u
      if  ;; label = @2
        local.get 0
        local.set 6
        br 1 (;@1;)
      end
      loop  ;; label = @2
        local.get 4
        i32.const 9
        i32.add
        local.get 2
        i32.add
        local.tee 3
        i32.const -2
        i32.add
        local.get 0
        i64.const 10000
        i64.rem_u
        i32.wrap_i64
        local.tee 5
        i32.const 100
        i32.rem_u
        i32.const 1
        i32.shl
        i32.const 10412
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        local.get 3
        i32.const -4
        i32.add
        local.get 5
        i32.const 100
        i32.div_u
        i32.const 1
        i32.shl
        i32.const 10412
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        local.get 2
        i32.const -4
        i32.add
        local.set 2
        local.get 0
        i64.const 99999999
        i64.gt_u
        local.get 0
        i64.const 10000
        i64.div_u
        local.tee 6
        local.set 0
        br_if 0 (;@2;)
      end
    end
    block  ;; label = @1
      local.get 6
      i32.wrap_i64
      local.tee 3
      i32.const 99
      i32.gt_s
      if  ;; label = @2
        local.get 2
        i32.const -2
        i32.add
        local.tee 2
        local.get 4
        i32.const 9
        i32.add
        i32.add
        local.get 6
        i32.wrap_i64
        i32.const 65535
        i32.and
        local.tee 3
        i32.const 100
        i32.rem_u
        i32.const 1
        i32.shl
        i32.const 10412
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        local.get 3
        i32.const 100
        i32.div_u
        local.set 3
      end
      local.get 3
      i32.const 10
      i32.ge_s
      if  ;; label = @2
        local.get 2
        i32.const -2
        i32.add
        local.tee 2
        local.get 4
        i32.const 9
        i32.add
        i32.add
        local.get 3
        i32.const 1
        i32.shl
        i32.const 10412
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        br 1 (;@1;)
      end
      local.get 2
      i32.const -1
      i32.add
      local.tee 2
      local.get 4
      i32.const 9
      i32.add
      i32.add
      local.get 3
      i32.const 48
      i32.add
      i32.store8
    end
    local.get 1
    local.get 4
    i32.const 9
    i32.add
    local.get 2
    i32.add
    i32.const 39
    local.get 2
    i32.sub
    call 20
    local.get 4
    i32.const 48
    i32.add
    global.set 0)
  (func (;20;) (type 10) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 4
    global.set 0
    i32.const 43
    i32.const 1114112
    local.get 0
    i32.load
    local.tee 5
    i32.const 1
    i32.and
    local.tee 3
    select
    local.set 6
    local.get 2
    local.get 3
    i32.add
    local.set 7
    i32.const 11096
    i32.const 0
    local.get 5
    i32.const 4
    i32.and
    select
    local.set 3
    i32.const 1
    local.set 8
    block  ;; label = @1
      local.get 0
      i32.load offset=8
      i32.const 1
      i32.ne
      if  ;; label = @2
        local.get 0
        local.get 6
        local.get 3
        call 25
        br_if 1 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 1
        local.get 2
        local.get 0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 1)
        local.set 8
        br 1 (;@1;)
      end
      local.get 0
      i32.const 12
      i32.add
      i32.load
      local.tee 9
      local.get 7
      i32.le_u
      if  ;; label = @2
        local.get 0
        local.get 6
        local.get 3
        call 25
        br_if 1 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 1
        local.get 2
        local.get 0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 1)
        local.set 8
        br 1 (;@1;)
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 5
          i32.const 8
          i32.and
          if  ;; label = @4
            local.get 0
            i32.load offset=4
            local.set 5
            local.get 0
            i32.const 48
            i32.store offset=4
            local.get 0
            i32.load8_u offset=32
            local.set 10
            local.get 0
            i32.const 1
            i32.store8 offset=32
            local.get 0
            local.get 6
            local.get 3
            call 25
            br_if 3 (;@1;)
            local.get 4
            i32.const 8
            i32.add
            local.get 0
            local.get 9
            local.get 7
            i32.sub
            call 26
            local.get 4
            i32.load offset=8
            local.tee 6
            i32.const 1114112
            i32.eq
            br_if 3 (;@1;)
            local.get 4
            i32.load offset=12
            local.get 0
            i32.load offset=24
            local.get 1
            local.get 2
            local.get 0
            i32.const 28
            i32.add
            i32.load
            i32.load offset=12
            call_indirect (type 1)
            br_if 3 (;@1;)
            i32.const 1
            i32.add
            local.set 1
            local.get 0
            i32.load offset=28
            local.set 2
            local.get 0
            i32.load offset=24
            local.set 3
            loop  ;; label = @5
              local.get 1
              i32.const -1
              i32.add
              local.tee 1
              i32.eqz
              br_if 2 (;@3;)
              local.get 3
              local.get 6
              local.get 2
              i32.load offset=16
              call_indirect (type 2)
              i32.eqz
              br_if 0 (;@5;)
            end
            br 3 (;@1;)
          end
          local.get 4
          local.get 0
          local.get 9
          local.get 7
          i32.sub
          call 26
          local.get 4
          i32.load
          local.tee 7
          i32.const 1114112
          i32.eq
          br_if 2 (;@1;)
          local.get 4
          i32.load offset=4
          local.get 0
          local.get 6
          local.get 3
          call 25
          br_if 2 (;@1;)
          local.get 0
          i32.load offset=24
          local.get 1
          local.get 2
          local.get 0
          i32.const 28
          i32.add
          i32.load
          i32.load offset=12
          call_indirect (type 1)
          br_if 2 (;@1;)
          i32.const 1
          i32.add
          local.set 1
          local.get 0
          i32.load offset=28
          local.set 2
          local.get 0
          i32.load offset=24
          local.set 0
          loop  ;; label = @4
            local.get 1
            i32.const -1
            i32.add
            local.tee 1
            i32.eqz
            br_if 2 (;@2;)
            local.get 0
            local.get 7
            local.get 2
            i32.load offset=16
            call_indirect (type 2)
            i32.eqz
            br_if 0 (;@4;)
          end
          br 2 (;@1;)
        end
        local.get 0
        local.get 10
        i32.store8 offset=32
        local.get 0
        local.get 5
        i32.store offset=4
      end
      i32.const 0
      local.set 8
    end
    local.get 4
    i32.const 16
    i32.add
    global.set 0
    local.get 8)
  (func (;21;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i64.load32_u
    local.get 1
    call 19)
  (func (;22;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    drop
    loop  ;; label = @1
      br 0 (;@1;)
    end
    unreachable)
  (func (;23;) (type 3) (param i32 i32 i32 i32)
    (local i32 i32)
    block  ;; label = @1
      block (result i32)  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 3
            i32.load
            i32.const 1
            i32.sub
            br_table 1 (;@3;) 3 (;@1;) 0 (;@4;)
          end
          local.get 3
          i32.load offset=4
          br 1 (;@2;)
        end
        local.get 3
        i32.load offset=4
        local.tee 3
        local.get 2
        i32.ge_u
        if  ;; label = @3
          local.get 3
          local.get 2
          i32.const 10740
          call 24
          unreachable
        end
        local.get 1
        local.get 3
        i32.const 3
        i32.shl
        i32.add
        local.tee 1
        i32.load offset=4
        i32.const 2
        i32.ne
        br_if 1 (;@1;)
        local.get 1
        i32.load
        i32.load
      end
      local.set 5
      i32.const 1
      local.set 4
    end
    local.get 0
    local.get 5
    i32.store offset=4
    local.get 0
    local.get 4
    i32.store)
  (func (;24;) (type 5) (param i32 i32 i32)
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
    call 27
    unreachable)
  (func (;25;) (type 10) (param i32 i32 i32) (result i32)
    block (result i32)  ;; label = @1
      local.get 1
      i32.const 1114112
      i32.ne
      if  ;; label = @2
        i32.const 1
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
        drop
      end
      local.get 2
      i32.eqz
      if  ;; label = @2
        i32.const 0
        return
      end
      local.get 0
      i32.load offset=24
      local.get 2
      i32.const 0
      local.get 0
      i32.const 28
      i32.add
      i32.load
      i32.load offset=12
      call_indirect (type 1)
    end)
  (func (;26;) (type 12) (param i32 i32 i32)
    (local i32 i32 i32)
    local.get 2
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          i32.const 1
          local.get 1
          i32.load8_u offset=32
          local.tee 5
          local.get 5
          i32.const 3
          i32.eq
          select
          i32.const 255
          i32.and
          i32.const 1
          i32.sub
          br_table 1 (;@2;) 0 (;@3;) 1 (;@2;) 2 (;@1;)
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
        local.set 3
        br 1 (;@1;)
      end
      i32.const 0
      local.set 3
      local.get 2
      local.set 4
    end
    local.get 4
    i32.const 1
    i32.add
    local.set 2
    block (result i32)  ;; label = @1
      loop  ;; label = @2
        local.get 2
        i32.const -1
        i32.add
        local.tee 2
        i32.eqz
        if  ;; label = @3
          local.get 1
          i32.load offset=4
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
    end
    local.set 1
    local.get 0
    local.get 3
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func (;27;) (type 0) (param i32 i32)
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
    call 28
    unreachable)
  (func (;28;) (type 6) (param i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 0
    i32.load offset=12
    local.set 3
    local.get 0
    i32.load offset=8
    local.tee 2
    i32.eqz
    if  ;; label = @1
      i32.const 10984
      i32.const 43
      i32.const 10952
      call 10
      unreachable
    end
    local.get 1
    local.get 3
    i32.store offset=8
    local.get 1
    local.get 0
    i32.store offset=4
    local.get 1
    local.get 2
    i32.store
    local.get 1
    call 30
    unreachable)
  (func (;29;) (type 7) (param i32) (result i64)
    i64.const 7504629734305773520)
  (func (;30;) (type 6) (param i32)
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
    global.get 0
    i32.const 16
    i32.sub
    global.set 0
    local.get 1
    i32.load offset=4
    i32.load offset=8
    local.get 1
    i32.load offset=8
    call 35
    unreachable)
  (func (;31;) (type 0) (param i32 i32)
    (local i32 i32 i32)
    local.get 1
    call 32
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
    i32.const 12
    i32.const 4
    call 8
    local.tee 1
    i32.eqz
    if  ;; label = @1
      i32.const 12
      i32.const 4
      call 9
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
  (func (;32;) (type 4) (param i32) (result i32)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const -64
    i32.add
    local.tee 1
    global.set 0
    local.get 0
    i32.const 4
    i32.add
    local.set 3
    local.get 0
    i32.load offset=4
    i32.eqz
    if  ;; label = @1
      local.get 0
      i32.load
      local.set 2
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
      i32.const 56
      i32.add
      local.get 2
      i32.const 16
      i32.add
      i64.load align=4
      i64.store
      local.get 1
      i32.const 48
      i32.add
      local.get 2
      i32.const 8
      i32.add
      i64.load align=4
      i64.store
      local.get 1
      local.get 2
      i64.load align=4
      i64.store offset=40
      local.get 1
      i32.const 36
      i32.add
      local.get 1
      i32.const 40
      i32.add
      call 17
      drop
      local.get 1
      i32.const 16
      i32.add
      local.tee 2
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
        call 7
      end
      local.get 3
      local.get 1
      i64.load offset=8
      i64.store align=4
      local.get 3
      i32.const 8
      i32.add
      local.get 2
      i32.load
      i32.store
    end
    local.get 1
    i32.const -64
    i32.sub
    global.set 0
    local.get 3)
  (func (;33;) (type 6) (param i32)
    (local i32)
    local.get 0
    i32.load offset=4
    local.tee 1
    if  ;; label = @1
      local.get 0
      i32.load
      local.get 1
      call 7
    end)
  (func (;34;) (type 0) (param i32 i32)
    local.get 1
    call 32
    local.set 1
    local.get 0
    i32.const 10968
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func (;35;) (type 9) (param i32 i32)
    i32.const 12148
    i32.const 12148
    i32.load
    i32.const 1
    i32.add
    i32.store
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          i32.const 12152
          i32.load
          local.tee 0
          i32.const 1
          i32.eq
          if  ;; label = @4
            i32.const 12156
            i32.const 0
            local.get 0
            i32.const 1
            i32.eq
            select
            local.tee 0
            local.get 0
            i32.load
            i32.const 1
            i32.add
            local.tee 0
            i32.store
            local.get 0
            i32.const 2
            i32.gt_u
            br_if 3 (;@1;)
            i32.const 12160
            i32.load
            local.tee 1
            i32.const -1
            i32.gt_s
            br_if 1 (;@3;)
            br 3 (;@1;)
          end
          i32.const 12152
          i64.const 1
          i64.store
          i32.const 12156
          i32.const 1
          i32.store
          i32.const 12160
          i32.load
          local.tee 0
          i32.const 0
          i32.lt_s
          br_if 2 (;@1;)
          i32.const 12160
          local.get 0
          i32.store
          br 1 (;@2;)
        end
        i32.const 12160
        local.get 1
        i32.store
        local.get 0
        i32.const 1
        i32.gt_u
        br_if 1 (;@1;)
      end
      unreachable
    end
    unreachable)
  (func (;36;) (type 6) (param i32)
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
      call 7
    end)
  (func (;37;) (type 3) (param i32 i32 i32 i32)
    block (result i32)  ;; label = @1
      local.get 2
      i32.const 2
      i32.shl
      local.tee 1
      local.get 3
      i32.const 3
      i32.shl
      i32.const 16384
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
      i32.eq
      if  ;; label = @2
        i32.const 0
        local.set 3
        i32.const 1
        br 1 (;@1;)
      end
      local.get 2
      i32.const 16
      i32.shl
      local.tee 3
      i64.const 0
      i64.store offset=4 align=4
      local.get 3
      local.get 3
      local.get 1
      i32.const -65536
      i32.and
      i32.add
      i32.const 2
      i32.or
      i32.store
      i32.const 0
    end
    local.set 2
    local.get 0
    local.get 3
    i32.store offset=4
    local.get 0
    local.get 2
    i32.store)
  (func (;38;) (type 2) (param i32 i32) (result i32)
    i32.const 512)
  (func (;39;) (type 4) (param i32) (result i32)
    i32.const 1)
  (func (;40;) (type 8) (param i32 i32 i32 i32 i32) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 6
    global.set 0
    block  ;; label = @1
      local.get 0
      local.get 1
      local.get 2
      local.get 3
      local.get 4
      call 41
      local.tee 5
      br_if 0 (;@1;)
      local.get 6
      i32.const 8
      i32.add
      local.get 3
      local.get 0
      local.get 1
      local.get 4
      i32.load offset=12
      call_indirect (type 3)
      i32.const 0
      local.set 5
      local.get 6
      i32.load offset=8
      br_if 0 (;@1;)
      local.get 6
      i32.load offset=12
      local.tee 5
      local.get 2
      i32.load
      i32.store offset=8
      local.get 2
      local.get 5
      i32.store
      local.get 0
      local.get 1
      local.get 2
      local.get 3
      local.get 4
      call 41
      local.set 5
    end
    local.get 6
    i32.const 16
    i32.add
    global.set 0
    local.get 5)
  (func (;41;) (type 8) (param i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    local.get 2
    i32.load
    local.tee 5
    if  ;; label = @1
      local.get 1
      i32.const -1
      i32.add
      local.set 10
      local.get 0
      i32.const 2
      i32.shl
      local.set 9
      i32.const 0
      local.get 1
      i32.sub
      local.set 11
      loop  ;; label = @2
        local.get 5
        i32.const 8
        i32.add
        local.set 6
        local.get 5
        i32.load offset=8
        local.tee 7
        i32.const 1
        i32.and
        if  ;; label = @3
          loop  ;; label = @4
            local.get 6
            local.get 7
            i32.const -2
            i32.and
            i32.store
            block (result i32)  ;; label = @5
              i32.const 0
              local.get 5
              i32.load offset=4
              local.tee 7
              i32.const -4
              i32.and
              local.tee 6
              i32.eqz
              br_if 0 (;@5;)
              drop
              i32.const 0
              local.get 6
              local.get 6
              i32.load8_u
              i32.const 1
              i32.and
              select
            end
            local.set 1
            block  ;; label = @5
              local.get 5
              i32.load
              local.tee 8
              i32.const -4
              i32.and
              local.tee 12
              i32.eqz
              br_if 0 (;@5;)
              i32.const 0
              local.get 12
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
              local.get 6
              i32.or
              i32.store offset=4
              local.get 5
              i32.load offset=4
              local.tee 7
              i32.const -4
              i32.and
              local.set 6
            end
            local.get 5
            local.get 6
            if (result i32)  ;; label = @5
              local.get 6
              local.get 6
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
            else
              local.get 7
            end
            i32.const 3
            i32.and
            i32.store offset=4
            local.get 5
            local.get 5
            i32.load
            local.tee 5
            i32.const 3
            i32.and
            i32.store
            local.get 5
            i32.const 2
            i32.and
            if  ;; label = @5
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
            local.set 6
            local.get 1
            local.tee 5
            i32.load offset=8
            local.tee 7
            i32.const 1
            i32.and
            br_if 0 (;@4;)
          end
        end
        block  ;; label = @3
          local.get 5
          i32.load
          i32.const -4
          i32.and
          local.tee 1
          local.get 6
          i32.sub
          local.get 9
          i32.lt_u
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 6
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
            local.get 9
            i32.sub
            local.get 11
            i32.and
            local.tee 1
            i32.gt_u
            if  ;; label = @5
              local.get 6
              local.get 10
              i32.and
              br_if 2 (;@3;)
              local.get 2
              local.get 6
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
              local.tee 0
              i32.const -4
              i32.and
              local.tee 2
              i32.eqz
              br_if 0 (;@5;)
              i32.const 0
              local.get 2
              local.get 0
              i32.const 2
              i32.and
              select
              local.tee 0
              i32.eqz
              br_if 0 (;@5;)
              local.get 0
              local.get 0
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
            local.get 6
            local.get 6
            i32.load
            i32.const -2
            i32.and
            i32.store
            local.get 5
            i32.load
            local.tee 0
            i32.const 2
            i32.and
            if  ;; label = @5
              local.get 5
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
        local.get 5
        i32.load offset=8
        local.tee 5
        i32.store
        local.get 5
        br_if 0 (;@2;)
      end
    end
    i32.const 0)
  (func (;42;) (type 3) (param i32 i32 i32 i32)
    (local i32 i32 i32)
    local.get 0
    i32.load
    local.tee 4
    i32.const 0
    i32.store
    local.get 4
    i32.const -8
    i32.add
    local.tee 5
    local.get 5
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
          local.get 4
          i32.const -4
          i32.add
          local.tee 3
          i32.load
          i32.const -4
          i32.and
          local.tee 0
          if  ;; label = @4
            local.get 0
            i32.load
            local.tee 6
            i32.const 1
            i32.and
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 5
          i32.load
          local.tee 0
          i32.const -4
          i32.and
          local.tee 2
          i32.eqz
          br_if 1 (;@2;)
          i32.const 0
          local.get 2
          local.get 0
          i32.const 2
          i32.and
          select
          local.tee 0
          i32.eqz
          br_if 1 (;@2;)
          local.get 0
          i32.load8_u
          i32.const 1
          i32.and
          br_if 1 (;@2;)
          local.get 4
          local.get 0
          i32.load offset=8
          i32.const -4
          i32.and
          i32.store
          local.get 0
          local.get 5
          i32.const 1
          i32.or
          i32.store offset=8
          return
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 5
            i32.load
            local.tee 4
            i32.const -4
            i32.and
            local.tee 2
            i32.eqz
            if  ;; label = @5
              local.get 0
              local.set 1
              br 1 (;@4;)
            end
            local.get 0
            local.set 1
            i32.const 0
            local.get 2
            local.get 4
            i32.const 2
            i32.and
            select
            local.tee 4
            i32.eqz
            br_if 0 (;@4;)
            local.get 4
            local.get 4
            i32.load offset=4
            i32.const 3
            i32.and
            local.get 0
            i32.or
            i32.store offset=4
            local.get 3
            i32.load
            local.tee 2
            i32.const -4
            i32.and
            local.tee 1
            i32.eqz
            br_if 1 (;@3;)
            local.get 5
            i32.load
            i32.const -4
            i32.and
            local.set 2
            local.get 1
            i32.load
            local.set 6
          end
          local.get 1
          local.get 6
          i32.const 3
          i32.and
          local.get 2
          i32.or
          i32.store
          local.get 3
          i32.load
          local.set 2
        end
        local.get 3
        local.get 2
        i32.const 3
        i32.and
        i32.store
        local.get 5
        local.get 5
        i32.load
        local.tee 1
        i32.const 3
        i32.and
        i32.store
        local.get 1
        i32.const 2
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        local.get 0
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
      local.get 5
      i32.store
    end)
  (func (;43;) (type 3) (param i32 i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
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
    local.tee 4
    i32.const 4
    local.get 3
    i32.const 12
    i32.add
    i32.const 11096
    i32.const 11096
    call 40
    local.set 2
    local.get 1
    local.get 3
    i32.load offset=12
    i32.store
    local.get 2
    if (result i32)  ;; label = @1
      local.get 2
      i64.const 0
      i64.store offset=4 align=4
      local.get 2
      local.get 2
      local.get 4
      i32.const 2
      i32.shl
      i32.add
      i32.const 2
      i32.or
      i32.store
      i32.const 0
    else
      i32.const 1
    end
    local.set 1
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    local.get 3
    i32.const 16
    i32.add
    global.set 0)
  (func (;44;) (type 2) (param i32 i32) (result i32)
    local.get 1)
  (func (;45;) (type 4) (param i32) (result i32)
    i32.const 0)
  (func (;46;) (type 12) (param i32 i32 i32)
    local.get 2
    if  ;; label = @1
      loop  ;; label = @2
        local.get 0
        local.get 1
        i32.load8_u
        i32.store8
        local.get 0
        i32.const 1
        i32.add
        local.set 0
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
    end)
  (table (;0;) 24 24 funcref)
  (memory (;0;) 1)
  (global (;0;) (mut i32) (i32.const 10240))
  (global (;1;) i32 (i32.const 12168))
  (global (;2;) i32 (i32.const 12168))
  (export "memory" (memory 0))
  (export "main" (func 3))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (elem (;0;) (i32.const 1) func 12 22 21 13 14 15 16 13 18 33 29 36 31 34 13 43 44 45 13 37 38 39 13)
  (data (;0;) (i32.const 10240) "capacity overflow\00\00\00$(\00\00p\00\00\00\1d\02\00\00\05\00\00\00/home/sffc/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec.rs\04\00\00\00\04\00\00\00\04\00\00\00\05\00\00\00\06\00\00\00\07\00\00\0000010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899\84)\00\00o\00\00\00S\04\00\00\11\00\00\00/home/sffc/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/fmt/mod.rs\00\84)\00\00o\00\00\00]\04\00\00$\00\00\00\08\00\00\00\00\00\00\00\01\00\00\00\09\00\00\00$*\00\00 \00\00\00D*\00\00\12\00\00\00index out of bounds: the len is  but the index is /home/sffc/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs\00\00V*\00\00p\00\00\00\e2\01\00\00\1e\00\00\00\0a\00\00\00\0c\00\00\00\04\00\00\00\0b\00\00\00called `Option::unwrap()` on a `None` value\00\0c\00\00\00\10\00\00\00\04\00\00\00\0d\00\00\00\0e\00\00\00\0f\00\00\00\04\00\00\00\04\00\00\00\10\00\00\00\11\00\00\00\12\00\00\00\13\00\00\00\00\00\00\00\01\00\00\00\14\00\00\00\15\00\00\00\16\00\00\00\17\00\00\00\00\00\00\00\01\00\00\00\14\00\00\00\15\00\00\00\16"))
