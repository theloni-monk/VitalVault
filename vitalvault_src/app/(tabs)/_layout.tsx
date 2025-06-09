import "../../global.css"

import { Tabs } from "expo-router";
import Ionicons from '@expo/vector-icons/Ionicons';

export default function TabLayout() {
  return (
    <Tabs>
      <Tabs.Screen name="index" options ={{title:"About",
          tabBarIcon: ({ color, focused }) => (
            <Ionicons name={focused ? 'information-circle' : 'information-circle-outline'} color={color} size={24}/>
          ),}}/>
      <Tabs.Screen name="injest" options ={{title:"Upload"}}/>
      <Tabs.Screen name="manage" options ={{title:"Manage"}}/>
      <Tabs.Screen name="read" options ={{title:"Read"}}/>
      <Tabs.Screen name="find" options ={{title:"Find"}}/>
    </Tabs>
  );
}
