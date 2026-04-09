let triple_dollar = """
  payload ${Map.get(meta, {id: 1})}
  """
let hash_nested = "payload #{Map.get(meta, {id: 1})}"
let plain = "plain string"
