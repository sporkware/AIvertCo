import React, { useState, useEffect } from 'react';
import './App.css';

// Types for the simulation data
interface Agent {
  id: string;
  name: string;
  department: string;
  status: 'active' | 'idle' | 'busy';
  currentTask?: string;
  managerId?: string;
}

interface Department {
  name: string;
  agentCount: number;
  activeProjects: number;
  status: 'healthy' | 'warning' | 'critical';
}

interface SystemMetrics {
  totalAgents: number;
  activeProjects: number;
  systemHealth: number;
  incidentsToday: number;
  ticketsResolved: number;
  uptime: string;
}

interface SimulationEvent {
  id: string;
  timestamp: string;
  type: 'project' | 'incident' | 'support' | 'deployment' | 'security';
  description: string;
  department: string;
  severity?: 'low' | 'medium' | 'high' | 'critical';
}

const App: React.FC = () => {
  const [agents, setAgents] = useState<Agent[]>([]);
  const [departments, setDepartments] = useState<Department[]>([]);
  const [metrics, setMetrics] = useState<SystemMetrics>({
    totalAgents: 0,
    activeProjects: 0,
    systemHealth: 100,
    incidentsToday: 0,
    ticketsResolved: 0,
    uptime: '00:00:00'
  });
  const [events, setEvents] = useState<SimulationEvent[]>([]);
  const [isSimulationRunning, setIsSimulationRunning] = useState(false);

  // Simulate real-time data updates
  useEffect(() => {
    // Initialize with sample data
    const sampleAgents: Agent[] = [
      { id: '1', name: 'Sarah Chen', department: 'Engineering', status: 'active', currentTask: 'Code Review' },
      { id: '2', name: 'Mike Rodriguez', department: 'Sales', status: 'busy', currentTask: 'Client Meeting' },
      { id: '3', name: 'Jordan Smith', department: 'DevOps', status: 'active', currentTask: 'Server Maintenance' },
      { id: '4', name: 'Alex Thompson', department: 'InfoSec', status: 'idle' },
      { id: '5', name: 'Lisa Park', department: 'Networking', status: 'active', currentTask: 'Load Balancer Config' },
      { id: '6', name: 'David Wilson', department: 'Ops', status: 'busy', currentTask: 'Incident Response' },
    ];

    const sampleDepartments: Department[] = [
      { name: 'Engineering', agentCount: 4, activeProjects: 3, status: 'healthy' },
      { name: 'Sales', agentCount: 3, activeProjects: 2, status: 'healthy' },
      { name: 'DevOps', agentCount: 4, activeProjects: 1, status: 'healthy' },
      { name: 'InfoSec', agentCount: 3, activeProjects: 0, status: 'warning' },
      { name: 'Networking', agentCount: 2, activeProjects: 1, status: 'healthy' },
      { name: 'Ops', agentCount: 4, activeProjects: 0, status: 'healthy' },
    ];

    const sampleEvents: SimulationEvent[] = [
      {
        id: '1',
        timestamp: new Date().toISOString(),
        type: 'project',
        description: 'New customer website project assigned',
        department: 'Engineering'
      },
      {
        id: '2',
        timestamp: new Date(Date.now() - 300000).toISOString(),
        type: 'incident',
        description: 'High CPU usage on web servers',
        department: 'DevOps',
        severity: 'high'
      },
      {
        id: '3',
        timestamp: new Date(Date.now() - 600000).toISOString(),
        type: 'support',
        description: 'Customer reported slow loading times',
        department: 'Ops',
        severity: 'medium'
      },
    ];

    setAgents(sampleAgents);
    setDepartments(sampleDepartments);
    setEvents(sampleEvents);

    // Simulate real-time updates
    const interval = setInterval(() => {
      // Random status updates
      setAgents(prev => prev.map(agent => ({
        ...agent,
        status: Math.random() > 0.8 ? (agent.status === 'active' ? 'busy' : 'active') : agent.status
      })));

      // Update metrics
      setMetrics(prev => ({
        ...prev,
        uptime: new Date(Date.now() - Math.random() * 86400000).toISOString().split('T')[1].split('.')[0],
        incidentsToday: prev.incidentsToday + (Math.random() > 0.95 ? 1 : 0),
        ticketsResolved: prev.ticketsResolved + (Math.random() > 0.9 ? 1 : 0)
      }));

      // Add random events
      if (Math.random() > 0.85) {
        const eventTypes: SimulationEvent['type'][] = ['project', 'incident', 'support', 'deployment', 'security'];
        const departments = ['Engineering', 'DevOps', 'InfoSec', 'Networking', 'Ops', 'Sales'];
        const severities: SimulationEvent['severity'][] = ['low', 'medium', 'high', 'critical'];

        const newEvent: SimulationEvent = {
          id: Date.now().toString(),
          timestamp: new Date().toISOString(),
          type: eventTypes[Math.floor(Math.random() * eventTypes.length)],
          description: `Automated ${eventTypes[Math.floor(Math.random() * eventTypes.length)]} event`,
          department: departments[Math.floor(Math.random() * departments.length)],
          severity: severities[Math.floor(Math.random() * severities.length)]
        };

        setEvents(prev => [newEvent, ...prev.slice(0, 9)]); // Keep last 10 events
      }
    }, 5000);

    return () => clearInterval(interval);
  }, []);

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'healthy': return 'text-green-600';
      case 'warning': return 'text-yellow-600';
      case 'critical': return 'text-red-600';
      case 'active': return 'text-green-600';
      case 'busy': return 'text-blue-600';
      case 'idle': return 'text-gray-600';
      default: return 'text-gray-600';
    }
  };

  const getEventIcon = (type: SimulationEvent['type']) => {
    switch (type) {
      case 'project': return '📋';
      case 'incident': return '🚨';
      case 'support': return '🎫';
      case 'deployment': return '🚀';
      case 'security': return '🔒';
      default: return '📝';
    }
  };

  const getSeverityColor = (severity?: string) => {
    switch (severity) {
      case 'critical': return 'text-red-600 bg-red-100';
      case 'high': return 'text-orange-600 bg-orange-100';
      case 'medium': return 'text-yellow-600 bg-yellow-100';
      case 'low': return 'text-green-600 bg-green-100';
      default: return 'text-gray-600 bg-gray-100';
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 p-6">
      <div className="max-w-7xl mx-auto">
        {/* Header */}
        <div className="mb-8">
          <h1 className="text-3xl font-bold text-gray-900 mb-2">
            🤖 AI Company Simulation Dashboard
          </h1>
          <div className="flex items-center space-x-4 text-sm text-gray-600">
            <span className={`px-2 py-1 rounded-full text-xs font-medium ${
              isSimulationRunning ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'
            }`}>
              {isSimulationRunning ? '🟢 RUNNING' : '🔴 STOPPED'}
            </span>
            <span>Uptime: {metrics.uptime}</span>
            <span>Last Updated: {new Date().toLocaleTimeString()}</span>
          </div>
        </div>

        {/* System Metrics */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="p-2 bg-blue-100 rounded-lg">
                <span className="text-2xl">👥</span>
              </div>
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-600">Total Agents</p>
                <p className="text-2xl font-bold text-gray-900">{metrics.totalAgents || agents.length}</p>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="p-2 bg-green-100 rounded-lg">
                <span className="text-2xl">📋</span>
              </div>
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-600">Active Projects</p>
                <p className="text-2xl font-bold text-gray-900">{metrics.activeProjects}</p>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="p-2 bg-yellow-100 rounded-lg">
                <span className="text-2xl">🏥</span>
              </div>
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-600">System Health</p>
                <p className="text-2xl font-bold text-gray-900">{metrics.systemHealth}%</p>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-lg shadow p-6">
            <div className="flex items-center">
              <div className="p-2 bg-purple-100 rounded-lg">
                <span className="text-2xl">🎫</span>
              </div>
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-600">Tickets Resolved</p>
                <p className="text-2xl font-bold text-gray-900">{metrics.ticketsResolved}</p>
              </div>
            </div>
          </div>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
          {/* Department Overview */}
          <div className="bg-white rounded-lg shadow">
            <div className="p-6 border-b border-gray-200">
              <h2 className="text-lg font-semibold text-gray-900">🏢 Department Status</h2>
            </div>
            <div className="p-6">
              <div className="space-y-4">
                {departments.map((dept) => (
                  <div key={dept.name} className="flex items-center justify-between p-4 bg-gray-50 rounded-lg">
                    <div className="flex items-center space-x-3">
                      <span className="text-lg">
                        {dept.name === 'Engineering' ? '⚙️' :
                         dept.name === 'Sales' ? '💼' :
                         dept.name === 'DevOps' ? '🔧' :
                         dept.name === 'InfoSec' ? '🔒' :
                         dept.name === 'Networking' ? '🌐' : '🎫'}
                      </span>
                      <div>
                        <p className="font-medium text-gray-900">{dept.name}</p>
                        <p className="text-sm text-gray-600">{dept.agentCount} agents</p>
                      </div>
                    </div>
                    <div className="text-right">
                      <span className={`px-2 py-1 rounded-full text-xs font-medium ${getStatusColor(dept.status)}`}>
                        {dept.status.toUpperCase()}
                      </span>
                      <p className="text-sm text-gray-600 mt-1">{dept.activeProjects} active projects</p>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </div>

          {/* Agent Activity */}
          <div className="bg-white rounded-lg shadow">
            <div className="p-6 border-b border-gray-200">
              <h2 className="text-lg font-semibold text-gray-900">👥 Agent Activity</h2>
            </div>
            <div className="p-6">
              <div className="space-y-3 max-h-96 overflow-y-auto">
                {agents.map((agent) => (
                  <div key={agent.id} className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
                    <div className="flex items-center space-x-3">
                      <span className="text-lg">
                        {agent.department === 'Engineering' ? '👨‍💻' :
                         agent.department === 'Sales' ? '🤝' :
                         agent.department === 'DevOps' ? '🔧' :
                         agent.department === 'InfoSec' ? '🛡️' :
                         agent.department === 'Networking' ? '🌐' : '🎫'}
                      </span>
                      <div>
                        <p className="font-medium text-gray-900">{agent.name}</p>
                        <p className="text-sm text-gray-600">{agent.department}</p>
                      </div>
                    </div>
                    <div className="text-right">
                      <span className={`px-2 py-1 rounded-full text-xs font-medium ${getStatusColor(agent.status)}`}>
                        {agent.status.toUpperCase()}
                      </span>
                      {agent.currentTask && (
                        <p className="text-xs text-gray-600 mt-1 max-w-32 truncate">{agent.currentTask}</p>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </div>

        {/* Recent Events */}
        <div className="mt-8 bg-white rounded-lg shadow">
          <div className="p-6 border-b border-gray-200">
            <h2 className="text-lg font-semibold text-gray-900">📝 Recent Events</h2>
          </div>
          <div className="p-6">
            <div className="space-y-4 max-h-96 overflow-y-auto">
              {events.map((event) => (
                <div key={event.id} className="flex items-start space-x-3 p-4 bg-gray-50 rounded-lg">
                  <span className="text-lg">{getEventIcon(event.type)}</span>
                  <div className="flex-1">
                    <div className="flex items-center space-x-2 mb-1">
                      <p className="font-medium text-gray-900">{event.description}</p>
                      {event.severity && (
                        <span className={`px-2 py-1 rounded-full text-xs font-medium ${getSeverityColor(event.severity)}`}>
                          {event.severity.toUpperCase()}
                        </span>
                      )}
                    </div>
                    <div className="flex items-center space-x-4 text-sm text-gray-600">
                      <span>{event.department}</span>
                      <span>•</span>
                      <span>{new Date(event.timestamp).toLocaleTimeString()}</span>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>

        {/* Control Panel */}
        <div className="mt-8 bg-white rounded-lg shadow">
          <div className="p-6 border-b border-gray-200">
            <h2 className="text-lg font-semibold text-gray-900">🎛️ Control Panel</h2>
          </div>
          <div className="p-6">
            <div className="flex space-x-4">
              <button
                onClick={() => setIsSimulationRunning(!isSimulationRunning)}
                className={`px-4 py-2 rounded-lg font-medium ${
                  isSimulationRunning
                    ? 'bg-red-600 hover:bg-red-700 text-white'
                    : 'bg-green-600 hover:bg-green-700 text-white'
                }`}
              >
                {isSimulationRunning ? '⏹️ Stop Simulation' : '▶️ Start Simulation'}
              </button>
              <button className="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-medium">
                🔄 Reset Simulation
              </button>
              <button className="px-4 py-2 bg-gray-600 hover:bg-gray-700 text-white rounded-lg font-medium">
                📊 Export Report
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default App;