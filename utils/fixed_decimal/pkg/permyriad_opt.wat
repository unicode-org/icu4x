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
  (type (;10;) (func (param i32) (result i32)))
  (type (;11;) (func (param i32 i32)))
  (type (;12;) (func (param i32 i32 i32)))
  (func (;0;) (type 10) (param i32) (result i32)
    (local i32 i32)
    i32.const 1
    local.set 2
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
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
      local.tee 2
      i32.const -1
      i32.add
      local.tee 0
      i32.const 255
      i32.le_u
      if  ;; label = @2
        local.get 1
        i32.const 10480
        i32.store offset=4
        local.get 1
        local.get 0
        i32.const 2
        i32.shl
        i32.const 10484
        i32.add
        local.tee 0
        i32.load
        i32.store offset=12
        local.get 2
        i32.const 1
        local.get 1
        i32.const 12
        i32.add
        local.get 1
        i32.const 4
        i32.add
        i32.const 10408
        call 18
        local.set 2
        local.get 0
        local.get 1
        i32.load offset=12
        i32.store
        br 1 (;@1;)
      end
      local.get 1
      i32.const 10480
      i32.load
      i32.store offset=8
      local.get 2
      i32.const 1
      local.get 1
      i32.const 8
      i32.add
      i32.const 10456
      i32.const 10432
      call 18
      local.set 2
      i32.const 10480
      local.get 1
      i32.load offset=8
      i32.store
    end
    local.get 1
    i32.const 16
    i32.add
    global.set 0
    local.get 2)
  (func (;1;) (type 0) (param i32 i32)
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
        call 20
        local.get 0
        local.get 2
        i32.load offset=12
        i32.store
        br 1 (;@1;)
      end
      local.get 2
      i32.const 10480
      i32.load
      i32.store offset=12
      local.get 2
      i32.const 4
      i32.add
      local.get 2
      i32.const 12
      i32.add
      i32.const 10456
      i32.const 10432
      call 20
      i32.const 10480
      local.get 2
      i32.load offset=12
      i32.store
    end
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func (;2;) (type 4) (param i32 i32 i32) (result i32)
    (local i32)
    local.get 2
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
      call 24
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
    i32.const 1
    call 0
    local.tee 1
    if  ;; label = @1
      local.get 0
      i64.const 1
      i64.store offset=36 align=4
      local.get 0
      local.get 1
      i32.store offset=32
      i32.const 48
      local.set 1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.load16_s offset=24
          local.tee 2
          i32.const 0
          i32.lt_s
          br_if 0 (;@3;)
          block (result i32)  ;; label = @4
            local.get 0
            i32.const 8
            i32.add
            i32.const 5
            i32.or
            local.get 0
            i32.load offset=8
            local.tee 3
            i32.const 8
            i32.le_u
            br_if 0 (;@4;)
            drop
            local.get 0
            i32.load offset=20
            local.set 3
            local.get 0
            i32.load offset=16
          end
          local.tee 4
          i32.eqz
          local.get 3
          local.get 2
          i32.le_u
          i32.or
          br_if 0 (;@3;)
          local.get 2
          local.get 4
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
          br_if 0 (;@3;)
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
          call 4
          br 1 (;@2;)
        end
        local.get 0
        i32.load offset=40
        local.tee 2
        local.get 0
        i32.load offset=36
        i32.eq
        if  ;; label = @3
          local.get 0
          i32.const 32
          i32.add
          local.get 2
          i32.const 1
          call 5
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
      if  ;; label = @2
        local.get 0
        i32.load offset=32
        local.get 1
        call 6
      end
      local.get 0
      i32.load offset=8
      local.tee 1
      i32.const 8
      i32.gt_u
      if  ;; label = @2
        local.get 0
        i32.load offset=16
        local.get 1
        call 6
      end
      local.get 0
      i32.const 48
      i32.add
      global.set 0
      i32.const 0
      return
    end
    i32.const 1
    i32.const 1
    call 7
    unreachable)
  (func (;4;) (type 11) (param i32 i32)
    local.get 0
    local.get 0
    i32.load offset=8
    i32.const 2
    call 5
    local.get 0
    i32.load
    local.get 0
    i32.load offset=8
    i32.add
    local.get 1
    i32.const 2
    call 24
    local.get 0
    local.get 0
    i32.load offset=8
    i32.const 2
    i32.add
    i32.store offset=8)
  (func (;5;) (type 5) (param i32 i32 i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 0
      i32.load offset=4
      local.tee 3
      local.get 1
      i32.sub
      local.get 2
      i32.lt_u
      if  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            local.get 2
            i32.add
            local.tee 2
            local.get 1
            i32.ge_u
            if  ;; label = @5
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
              local.get 3
              i32.eqz
              i32.or
              br_if 1 (;@4;)
              local.get 1
              local.get 3
              local.get 2
              call 2
              local.tee 1
              br_if 2 (;@3;)
              i32.const 1
              local.set 4
              br 4 (;@1;)
            end
            br 3 (;@1;)
          end
          i32.const 1
          local.set 4
          local.get 2
          call 0
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
    local.get 4
    if  ;; label = @1
      local.get 2
      local.get 4
      call 7
      unreachable
    end
    i32.const 10240
    i32.const 17
    i32.const 10260
    call 9
    unreachable)
  (func (;6;) (type 0) (param i32 i32)
    local.get 0
    local.get 1
    call 1)
  (func (;7;) (type 0) (param i32 i32)
    local.get 0
    local.get 1
    i32.const 11524
    i32.load
    local.tee 0
    i32.const 1
    local.get 0
    select
    call_indirect (type 0)
    unreachable)
  (func (;8;) (type 0) (param i32 i32)
    nop)
  (func (;9;) (type 5) (param i32 i32 i32)
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
    call 10
    unreachable)
  (func (;10;) (type 0) (param i32 i32)
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
    call 11
    unreachable)
  (func (;11;) (type 7) (param i32)
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
    local.tee 3
    i32.eqz
    if  ;; label = @1
      i32.const 10320
      i32.const 43
      i32.const 10364
      call 9
      unreachable
    end
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
    i32.load
    drop
    call 14
    unreachable)
  (func (;12;) (type 7) (param i32)
    nop)
  (func (;13;) (type 8) (param i32) (result i64)
    i64.const 5966890128770411197)
  (func (;14;) (type 6)
    (local i32 i32)
    i32.const 11508
    i32.const 11508
    i32.load
    i32.const 1
    i32.add
    i32.store
    block  ;; label = @1
      block  ;; label = @2
        i32.const 11512
        i32.load
        i32.const 1
        i32.eq
        if  ;; label = @3
          i32.const 11516
          i32.const 11516
          i32.load
          i32.const 1
          i32.add
          local.tee 0
          i32.store
          local.get 0
          i32.const 2
          i32.gt_u
          br_if 2 (;@1;)
          i32.const 11520
          i32.load
          local.tee 1
          i32.const -1
          i32.gt_s
          br_if 1 (;@2;)
          br 2 (;@1;)
        end
        i32.const 11512
        i64.const 4294967297
        i64.store
        i32.const 11520
        i32.load
        local.tee 0
        i32.const 0
        i32.lt_s
        br_if 1 (;@1;)
        i32.const 11520
        local.get 0
        i32.store
        unreachable
      end
      i32.const 11520
      local.get 1
      i32.store
      local.get 0
      i32.const 1
      i32.gt_u
      br_if 0 (;@1;)
      unreachable
    end
    unreachable)
  (func (;15;) (type 1) (param i32 i32 i32 i32)
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
  (func (;16;) (type 2) (param i32 i32) (result i32)
    i32.const 512)
  (func (;17;) (type 3) (param i32) (result i32)
    i32.const 1)
  (func (;18;) (type 9) (param i32 i32 i32 i32 i32) (result i32)
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
      call 19
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
      call_indirect (type 1)
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
      call 19
      local.set 5
    end
    local.get 6
    i32.const 16
    i32.add
    global.set 0
    local.get 5)
  (func (;19;) (type 9) (param i32 i32 i32 i32 i32) (result i32)
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
  (func (;20;) (type 1) (param i32 i32 i32 i32)
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
        call_indirect (type 3)
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
  (func (;21;) (type 1) (param i32 i32 i32 i32)
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
    i32.const 10456
    i32.const 10456
    call 18
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
  (func (;22;) (type 2) (param i32 i32) (result i32)
    local.get 1)
  (func (;23;) (type 3) (param i32) (result i32)
    i32.const 0)
  (func (;24;) (type 12) (param i32 i32 i32)
    local.get 2
    if  ;; label = @1
      loop  ;; label = @2
        local.get 0
        local.get 1
        i32.load8_u
        i32.store8
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
        br_if 0 (;@2;)
      end
    end)
  (table (;0;) 13 13 funcref)
  (memory (;0;) 1)
  (global (;0;) (mut i32) (i32.const 10240))
  (global (;1;) i32 (i32.const 11528))
  (global (;2;) i32 (i32.const 11528))
  (export "memory" (memory 0))
  (export "main" (func 3))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (elem (;0;) (i32.const 1) func 8 12 13 12 21 22 23 12 15 16 17 12)
  (data (;0;) (i32.const 10240) "capacity overflow\00\00\00$(\00\00\1c\00\00\00\1d\02\00\00\05\00\00\00library/alloc/src/raw_vec.rs\02\00\00\00\00\00\00\00\01\00\00\00\03\00\00\00called `Option::unwrap()` on a `None` value\00\8c(\00\00\1c\00\00\00\e2\01\00\00\1e\00\00\00library/std/src/panicking.rs\04\00\00\00\04\00\00\00\04\00\00\00\05\00\00\00\06\00\00\00\07\00\00\00\08\00\00\00\00\00\00\00\01\00\00\00\09\00\00\00\0a\00\00\00\0b\00\00\00\0c\00\00\00\00\00\00\00\01\00\00\00\09\00\00\00\0a\00\00\00\0b"))
