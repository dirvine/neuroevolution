struct NeuralNetwork
{
    /////////////////////
    // RTRL variables
    double m_total_error;
    // Always the size of m_connections
    std::vector<double> m_total_weight_change;
    /////////////////////
    // returns the index if that connection exists or -1 otherwise
    int ConnectionExists(int a_to, int a_from);
    unsigned int m_num_inputs, m_num_outputs;
    std::vector<Connection> m_connections; // array size - number of connections
    std::vector<Neuron>     m_neurons;

    NeuralNetwork(bool a_Minimal); // if given false, the constructor will create a standard XOR network topology.
    NeuralNetwork();

    void InitRTRLMatrix(); // initializes the sensitivity cube for RTRL learning.
    // assumes that neuron and connection data are already initialized

    void ActivateFast();          // assumes unsigned sigmoids everywhere.
    void Activate();              // any activation functions are supported
    void ActivateUseInternalBias(); // like Activate() but uses m_bias as well
    void ActivateLeaky(double step); // activates in leaky integrator mode

    void RTRL_update_gradients();
    void RTRL_update_error(double a_target);
    void RTRL_update_weights();   // performs the backprop step

    // Hebbian learning
    void Adapt(Parameters& a_Parameters);

    void Flush();     // clears all activations
    void FlushCube(); // clears the sensitivity cube

    void Input(std::vector<double>& a_Inputs);

    std::vector<double> Output();

    // accessor methods
    void AddNeuron(const Neuron& a_n) { m_neurons.push_back( a_n ); }
    void AddConnection(const Connection& a_c) { m_connections.push_back( a_c ); }
    Connection GetConnectionByIndex(unsigned int a_idx) const
    {
        return m_connections[a_idx];
    }
    Neuron GetNeuronByIndex(unsigned int a_idx) const
    {
        return m_neurons[a_idx];
    }
    void SetInputOutputDimentions(const unsigned short a_i, const unsigned short a_o)
    {
        m_num_inputs = a_i;
        m_num_outputs = a_o;
    }
    unsigned int NumInputs() const
    {
        return m_num_inputs;
    }
    unsigned int NumOutputs() const
    {
        return m_num_outputs;
    }

    // clears the network and makes it a minimal one
    void Clear()
    {
        m_neurons.clear();
        m_connections.clear();
        m_total_weight_change.clear();
        SetInputOutputDimentions(0, 0);
    }

    double GetConnectionLenght(Neuron source, Neuron target)
    {   double dist = 0.0;
        for (unsigned int i = 0; i < source.m_substrate_coords.size(); i++)
        {
            dist += (target.m_substrate_coords[i] - source.m_substrate_coords[i]) *
            		(target.m_substrate_coords[i] - source.m_substrate_coords[i] );
        }
        return dist;
    }

    double GetTotalConnectionLength()
    {
        return m_connections.size();
    }

    // one-shot save/load
    void Save(const char* a_filename);
    bool Load(const char* a_filename);

    // save/load from already opened files for reading/writing
    void Save(FILE* a_file);
    bool Load(std::ifstream& a_DataFile);
}
