import { Text, View } from "react-native";
import "../../global.css"


export default function Manage() {
  //WRITEME: fetch files, allow group alisasing into folders - backend structure is flat
  return (
    <View
      className="bg-amber-50 flex-1 justify-top"
    >
      <Text className='text-blue-800 text-xl flex-col font-bold self-center w-3/4 pt-10' >manage stuff</Text>
    </View>
  );
}
