import { Text, View } from "react-native";
import "../../global.css"


export default function Read() {
  // get doc scope from tauriappstate, load prev and next in scope
  // swipe through docs in scope to read, autosyncs scope with manage tab
  return (
    <View
      className="bg-amber-50 flex-1 justify-top"
    >
      <Text className='text-blue-800 text-xl flex-col font-bold self-center w-3/4 pt-10' >Review your docs here</Text>
    </View>
  );
}
