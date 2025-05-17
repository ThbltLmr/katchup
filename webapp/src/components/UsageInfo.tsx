import { AlertCircle } from "lucide-react"

import {
  Alert,
  AlertDescription,
  AlertTitle,
} from "@/components/ui/alert"

export function UsageInfo() {
  return (
    <div className="px-4">
      <Alert className="text-start w-full">
        <AlertCircle className="h-4 w-4" />
        <AlertTitle>Info</AlertTitle>
        <AlertDescription className="text-sm">
          Select the next episode you will watch, and you will get a summary up to that point, excluding the selected episode.
        </AlertDescription>
      </Alert>
    </div>
  )
}

export default UsageInfo;

