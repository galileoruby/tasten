---
name: angular-websocket-components
description: Create Angular v21 components that use WebSocket connections directly instead of traditional services. Use this skill WHENEVER building components for Angular applications that communicate with Rust backends via WebSockets. Essential for real-time racing applications, live updates, and bidirectional communication. Trigger on mentions of 'component creation', 'WebSocket integration', 'real-time updates', 'live data', or any Angular component that needs direct socket communication.
compatibility: Angular v21+, TypeScript 5+, RxJS 7+
---

# Angular WebSocket Components Guide

Building Angular components for WebSocket-based communication requires a different architecture than traditional HTTP services. This skill guides the creation of components that manage socket connections directly with proper lifecycle management.

## Core Principles

### 1. Socket Connection at Component Level
Instead of delegating to a service layer, WebSocket connections are **initialized and managed within the component**. This provides:
- Direct access to socket events without service abstraction overhead
- Clearer lifecycle management tied to component destruction
- Better control over real-time updates

### 2. Lifecycle Management
WebSocket connections must be tightly coupled with Angular's component lifecycle:
- **ngOnInit**: Establish WebSocket connection
- **ngOnDestroy**: Close connection and unsubscribe from all observables
- **Change Detection**: Properly handle async updates from socket events

### 3. Message Protocol with Rust Backend
Define a clear message interface between Angular and Rust:
```typescript
// Message types sent from client
interface ClientMessage {
  type: 'join_race' | 'start_race' | 'position_update' | 'lap_complete';
  data: any;
  timestamp: number;
}

// Message types received from server
interface ServerMessage {
  type: 'race_started' | 'position_change' | 'lap_recorded' | 'race_ended';
  data: any;
  timestamp: number;
}
```

---

## Component Template Pattern

### Basic Structure
```typescript
import { Component, OnInit, OnDestroy, ChangeDetectorRef } from '@angular/core';
import { Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';

@Component({
  selector: 'app-race-monitor',
  templateUrl: './race-monitor.component.html',
  styleUrls: ['./race-monitor.component.css']
})
export class RaceMonitorComponent implements OnInit, OnDestroy {
  
  // WebSocket connection
  private socket: WebSocket | null = null;
  private socketUrl = 'ws://localhost:8080/races';
  
  // Component state
  raceData: any = null;
  isConnected = false;
  error: string | null = null;
  
  // Cleanup subscription
  private destroy$ = new Subject<void>();

  constructor(private cdr: ChangeDetectorRef) {}

  ngOnInit(): void {
    this.connectWebSocket();
  }

  ngOnDestroy(): void {
    this.disconnectWebSocket();
    this.destroy$.next();
    this.destroy$.complete();
  }

  private connectWebSocket(): void {
    try {
      this.socket = new WebSocket(this.socketUrl);

      // Connection established
      this.socket.onopen = (): void => {
        console.log('WebSocket connected');
        this.isConnected = true;
        this.error = null;
        this.cdr.markForCheck();
      };

      // Incoming message handler
      this.socket.onmessage = (event: MessageEvent): void => {
        this.handleServerMessage(event.data);
      };

      // Connection closed
      this.socket.onclose = (): void => {
        console.log('WebSocket disconnected');
        this.isConnected = false;
        this.cdr.markForCheck();
        // Auto-reconnect after 3 seconds
        setTimeout(() => this.connectWebSocket(), 3000);
      };

      // Error handling
      this.socket.onerror = (error: Event): void => {
        console.error('WebSocket error:', error);
        this.error = 'Connection error. Attempting to reconnect...';
        this.isConnected = false;
        this.cdr.markForCheck();
      };
    } catch (err) {
      console.error('Failed to connect:', err);
      this.error = 'Failed to establish connection';
    }
  }

  private handleServerMessage(rawData: string): void {
    try {
      const message = JSON.parse(rawData) as ServerMessage;

      switch (message.type) {
        case 'race_started':
          this.onRaceStarted(message.data);
          break;
        case 'position_change':
          this.onPositionChange(message.data);
          break;
        case 'lap_recorded':
          this.onLapRecorded(message.data);
          break;
        case 'race_ended':
          this.onRaceEnded(message.data);
          break;
        default:
          console.warn('Unknown message type:', message.type);
      }

      // Trigger change detection after state update
      this.cdr.markForCheck();
    } catch (err) {
      console.error('Failed to parse message:', err);
    }
  }

  private onRaceStarted(data: any): void {
    this.raceData = {
      ...this.raceData,
      status: 'running',
      startTime: data.timestamp
    };
  }

  private onPositionChange(data: any): void {
    if (this.raceData) {
      this.raceData.participants = data.participants;
    }
  }

  private onLapRecorded(data: any): void {
    if (this.raceData) {
      this.raceData.laps = [...(this.raceData.laps || []), data];
    }
  }

  private onRaceEnded(data: any): void {
    this.raceData = {
      ...this.raceData,
      status: 'completed',
      endTime: data.timestamp,
      winner: data.winner
    };
  }

  // Send message to server
  protected sendMessage(type: string, data: any): void {
    if (!this.socket || this.socket.readyState !== WebSocket.OPEN) {
      console.warn('Socket not ready. Current state:', this.socket?.readyState);
      return;
    }

    const message: ClientMessage = {
      type: type as any,
      data,
      timestamp: Date.now()
    };

    this.socket.send(JSON.stringify(message));
  }

  private disconnectWebSocket(): void {
    if (this.socket) {
      this.socket.close();
      this.socket = null;
    }
  }

  // Public methods for template interaction
  startRace(): void {
    this.sendMessage('start_race', { raceId: 'race-123' });
  }

  updatePosition(position: any): void {
    this.sendMessage('position_update', { ...position });
  }
}
```

---

## Template Example

```html
<div class="race-monitor">
  <!-- Connection Status -->
  <div class="status-bar" [class.connected]="isConnected">
    <span class="status-indicator" [class.active]="isConnected"></span>
    {{ isConnected ? 'Connected' : 'Disconnected' }}
  </div>

  <!-- Error Display -->
  <div *ngIf="error" class="error-banner">
    {{ error }}
  </div>

  <!-- Race Data Display -->
  <div *ngIf="raceData" class="race-content">
    <h2>{{ raceData.status }}</h2>
    
    <!-- Participants List -->
    <div class="participants">
      <div *ngFor="let participant of raceData.participants" 
           class="participant-card">
        <div class="participant-name">{{ participant.name }}</div>
        <div class="participant-position">Position: {{ participant.position }}</div>
        <div class="participant-lap">Lap: {{ participant.currentLap }}</div>
      </div>
    </div>

    <!-- Race Controls -->
    <div class="controls">
      <button (click)="startRace()" [disabled]="!isConnected">
        Start Race
      </button>
    </div>
  </div>

  <!-- Loading State -->
  <div *ngIf="!raceData && isConnected" class="loading">
    Waiting for race data...
  </div>
</div>
```

---

## Advanced Patterns

### 1. Message Queue for Offline Support
```typescript
private messageQueue: ClientMessage[] = [];

protected sendMessage(type: string, data: any): void {
  const message: ClientMessage = {
    type: type as any,
    data,
    timestamp: Date.now()
  };

  if (this.socket?.readyState === WebSocket.OPEN) {
    this.socket.send(JSON.stringify(message));
  } else {
    // Queue message while offline
    this.messageQueue.push(message);
  }
}

// Flush queue when reconnected
private socket.onopen = (): void => {
  this.isConnected = true;
  
  // Send queued messages
  while (this.messageQueue.length > 0) {
    const msg = this.messageQueue.shift();
    if (msg && this.socket?.readyState === WebSocket.OPEN) {
      this.socket.send(JSON.stringify(msg));
    }
  }
};
```

### 2. Heartbeat/Keep-Alive
```typescript
private heartbeatInterval: any;

private connectWebSocket(): void {
  // ... existing connection code ...
  
  this.socket!.onopen = (): void => {
    this.isConnected = true;
    
    // Send heartbeat every 30 seconds
    this.heartbeatInterval = setInterval(() => {
      this.sendMessage('ping', {});
    }, 30000);
  };
}

private disconnectWebSocket(): void {
  if (this.heartbeatInterval) {
    clearInterval(this.heartbeatInterval);
  }
  if (this.socket) {
    this.socket.close();
  }
}
```

### 3. Observable-Based Message Stream (Optional)
If you prefer RxJS for some flows:

```typescript
private messageSubject$ = new Subject<ServerMessage>();
public message$ = this.messageSubject$.asObservable();

private handleServerMessage(rawData: string): void {
  const message = JSON.parse(rawData) as ServerMessage;
  this.messageSubject$.next(message);
}

// In ngOnInit, subscribe to specific message types
this.message$
  .pipe(
    filter(msg => msg.type === 'position_change'),
    takeUntil(this.destroy$)
  )
  .subscribe(msg => this.onPositionChange(msg.data));
```

---

## Typescript Interfaces for Rust Backend

```typescript
// Define these in a shared types file
interface RaceParticipant {
  id: string;
  name: string;
  position: number;
  currentLap: number;
  lapTimes: number[];
  totalTime: number;
}

interface RaceSession {
  id: string;
  name: string;
  status: 'waiting' | 'running' | 'completed';
  participants: RaceParticipant[];
  startTime?: number;
  endTime?: number;
  winner?: string;
}

interface ClientMessage {
  type: 'join_race' | 'start_race' | 'position_update' | 'lap_complete' | 'ping';
  data: any;
  timestamp: number;
}

interface ServerMessage {
  type: 'race_started' | 'position_change' | 'lap_recorded' | 'race_ended' | 'pong';
  data: any;
  timestamp: number;
}
```

---

## Change Detection Strategy

Use **OnPush** change detection for performance:

```typescript
@Component({
  selector: 'app-race-monitor',
  templateUrl: './race-monitor.component.html',
  styleUrls: ['./race-monitor.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush  // 👈 Add this
})
export class RaceMonitorComponent implements OnInit, OnDestroy {
  // ... remember to call cdr.markForCheck() after state updates
}
```

---

## Common Issues & Solutions

| Issue | Solution |
|-------|----------|
| Connection drops frequently | Implement heartbeat/keep-alive and auto-reconnect logic |
| Memory leaks | Always unsubscribe in ngOnDestroy and close socket |
| Messages lost when offline | Implement message queue as shown in Advanced Patterns |
| Stale UI data | Use ChangeDetectorRef.markForCheck() after socket events |
| Multiple connections created | Check ngOnInit isn't called multiple times (verify module imports) |

---

## Testing WebSocket Components

```typescript
// Example test with mock WebSocket
import { TestBed, ComponentFixture } from '@angular/core/testing';

describe('RaceMonitorComponent', () => {
  let component: RaceMonitorComponent;
  let fixture: ComponentFixture<RaceMonitorComponent>;
  let mockSocket: any;

  beforeEach(async () => {
    mockSocket = {
      send: jasmine.createSpy('send'),
      close: jasmine.createSpy('close'),
      readyState: WebSocket.OPEN,
      onopen: null,
      onmessage: null,
      onerror: null,
      onclose: null
    };

    spyOn(window, 'WebSocket' as any).and.returnValue(mockSocket);

    await TestBed.configureTestingModule({
      declarations: [RaceMonitorComponent]
    }).compileComponents();

    fixture = TestBed.createComponent(RaceMonitorComponent);
    component = fixture.componentInstance;
  });

  it('should connect on init', () => {
    fixture.detectChanges();
    expect(window.WebSocket).toHaveBeenCalledWith('ws://localhost:8080/races');
  });

  it('should send message when socket is open', () => {
    fixture.detectChanges();
    mockSocket.readyState = WebSocket.OPEN;
    
    component['sendMessage']('start_race', { raceId: 'test' });
    
    expect(mockSocket.send).toHaveBeenCalled();
  });
});
```

---

## Rust Backend Coordination

### Expected Rust Handler Pattern

Your Rust backend should:

1. **Accept WebSocket connections** on the defined URL
2. **Parse incoming messages** as JSON with the ClientMessage interface
3. **Broadcast updates** to all connected clients with ServerMessage format
4. **Handle disconnections** gracefully

Example structure (using tokio-tungstenite):
```rust
// Pseudo-code for Rust backend
loop {
  match socket.recv().await {
    Some(Message::Text(text)) => {
      let client_msg: ClientMessage = serde_json::from_str(&text)?;
      match client_msg.type {
        "join_race" => handle_join_race(client_msg.data),
        "start_race" => broadcast_to_all(ServerMessage { 
          type: "race_started",
          data: race_data,
          timestamp: now()
        }),
        // ... handle other message types
      }
    }
  }
}
```

The key is ensuring message types and data structures match between Angular and Rust implementations.

