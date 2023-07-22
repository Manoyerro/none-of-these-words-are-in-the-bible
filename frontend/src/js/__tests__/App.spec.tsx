import { render } from '@testing-library/react'
import App from '../App'
import React from 'react'

describe('<App/>', function () {
  it('renders successfully', () => {
    const { container } = render(<App/>)
    expect(container).toBeInTheDocument()
  })
})
